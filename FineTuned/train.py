#!/usr/bin/env python3
import os
# Set CUDA allocation configuration to allow expandable segments
os.environ["PYTORCH_CUDA_ALLOC_CONF"] = "expandable_segments:True"

# --- Colab-specific: authenticate and mount Google Drive ---
# try:
#     from google.colab import drive, auth
#     print("Authenticating user...")
#     auth.authenticate_user()  # Explicitly authenticate
#     print("Mounting Google Drive...")
#     drive.mount('/content/drive', force_remount=True)
#     DEFAULT_OUTPUT_DIR = "/content/drive/MyDrive/llama2-improved"
# except Exception as e:
#     print("Google Drive mounting failed:", e)
#     DEFAULT_OUTPUT_DIR = "./llama2-improved"

import torch
import argparse
from transformers import (
    AutoModelForCausalLM,
    AutoTokenizer,
    TrainingArguments,
    Trainer,
    DataCollatorForLanguageModeling,
    EarlyStoppingCallback
)
from datasets import Dataset
from peft import LoraConfig, get_peft_model
import wandb
import random
import numpy as np

def seed_everything(seed=42):
    """Set seeds for reproducibility."""
    random.seed(seed)
    np.random.seed(seed)
    torch.manual_seed(seed)
    if torch.cuda.is_available():
        torch.cuda.manual_seed_all(seed)
    os.environ['PYTHONHASHSEED'] = str(seed)

def load_data(input_file, output_file, test_size=0.05):
    """Load data with improved formatting for context handling."""
    with open(input_file, 'r', encoding='utf-8') as f_in:
        inputs = [line.strip() for line in f_in]
    with open(output_file, 'r', encoding='utf-8') as f_out:
        outputs = [line.strip() for line in f_out]
    assert len(inputs) == len(outputs), "Input and output counts must match"
    
    data = []
    for i in range(len(inputs)):
        input_text = inputs[i]
        formatted_text = f"### Instruction:\n{input_text}\n\n### Response:\n{outputs[i]}"
        data.append({
            "input": inputs[i],
            "output": outputs[i],
            "text": formatted_text
        })
    
    random.shuffle(data)
    split_idx = int(len(data) * (1 - test_size))
    train_data = data[:split_idx]
    val_data = data[split_idx:]
    
    return Dataset.from_list(train_data), Dataset.from_list(val_data)

def load_model(model_id, device="cpu"):
    """Load model with optimized configuration for context learning."""
    try:
        print(f"Loading tokenizer from {model_id}")
        tokenizer = AutoTokenizer.from_pretrained(model_id, use_fast=True, trust_remote_code=True)
        if tokenizer.pad_token is None:
            tokenizer.pad_token = tokenizer.eos_token
        
        load_options = {
            "cache_dir": "./model_cache",
            "low_cpu_mem_usage": True,
            "trust_remote_code": True,
        }
        
        # Use a sequential device map to offload parts of the model if necessary.
        if device == "cuda" and torch.cuda.is_available():
            load_options.update({
                "torch_dtype": torch.float16,
                "device_map": "sequential",
            })
        else:
            load_options.update({
                "torch_dtype": torch.float32,
            })
        
        if device == "cuda" and torch.cuda.is_available():
            torch.cuda.empty_cache()
        
        print(f"Loading model from {model_id}")
        model = AutoModelForCausalLM.from_pretrained(model_id, **load_options)
        model.config.use_cache = False
        
        print("Determining target modules for LoRA")
        if "llama" in model_id.lower():
            target_modules = ["q_proj", "k_proj", "v_proj", "o_proj"]
        elif "mistral" in model_id.lower():
            target_modules = ["q_proj", "k_proj", "v_proj", "o_proj"]
        elif "falcon" in model_id.lower():
            target_modules = ["query_key_value", "dense", "dense_h_to_4h", "dense_4h_to_h"]
        elif "tinyllama" in model_id.lower():
            target_modules = ["q_proj", "k_proj", "v_proj", "o_proj", "gate_proj", "up_proj", "down_proj"]
        else:
            target_modules = ["q_proj", "k_proj", "v_proj", "o_proj"]
            
        print(f"Using target modules: {target_modules}")
        model.train()
        
        lora_config = LoraConfig(
            r=8,
            lora_alpha=16,
            target_modules=target_modules,
            lora_dropout=0.05,
            bias="none",
            task_type="CAUSAL_LM",
        )
        
        print("Applying LoRA adapters")
        model = get_peft_model(model, lora_config)
        
        if hasattr(model, "enable_input_require_grads"):
            model.enable_input_require_grads()
        if hasattr(model, "gradient_checkpointing_enable"):
            print("Enabling gradient checkpointing")
            model.gradient_checkpointing_enable()
        
        model.train()
        
        print("Trainable parameters:")
        model.print_trainable_parameters()
        trainable_params = sum(p.numel() for p in model.parameters() if p.requires_grad)
        print(f"Total trainable parameters: {trainable_params}")
        
        if trainable_params == 0:
            raise ValueError("No trainable parameters found in the model")
        
        return model, tokenizer
    
    except Exception as e:
        print(f"Error loading model: {e}")
        print("Attempting to load a fallback model...")
        fallback_model = "TinyLlama/TinyLlama-1.1B-Chat-v1.0"
        tokenizer = AutoTokenizer.from_pretrained(fallback_model, trust_remote_code=True)
        tokenizer.pad_token = tokenizer.eos_token
        
        fallback_options = {
            "cache_dir": "./model_cache",
            "torch_dtype": torch.float16 if device == "cuda" else torch.float32,
            "low_cpu_mem_usage": True,
            "trust_remote_code": True,
        }
        if device == "cuda":
            fallback_options["device_map"] = "sequential"
        
        model = AutoModelForCausalLM.from_pretrained(fallback_model, **fallback_options)
        model.config.use_cache = False
        model.train()
        
        lora_config = LoraConfig(
            r=8,
            lora_alpha=16,
            target_modules=["q_proj", "k_proj", "v_proj", "o_proj"],
            lora_dropout=0.05,
            bias="none",
            task_type="CAUSAL_LM"
        )
        model = get_peft_model(model, lora_config)
        if hasattr(model, "enable_input_require_grads"):
            model.enable_input_require_grads()
        if hasattr(model, "gradient_checkpointing_enable"):
            model.gradient_checkpointing_enable()
        
        model.train()
        print("Trainable parameters in fallback model:")
        model.print_trainable_parameters()
        
        return model, tokenizer

def tokenize_function(examples, tokenizer, max_length=512):
    """Tokenize with improved handling for context examples."""
    results = tokenizer(
        examples["text"], 
        padding="max_length", 
        truncation=True, 
        max_length=max_length,
        return_tensors="pt"
    )
    results["labels"] = results["input_ids"].clone()
    pad_token_id = tokenizer.pad_token_id
    results["labels"] = [
        [(label if label != pad_token_id else -100) for label in labels]
        for labels in results["labels"]
    ]
    return results

def compute_metrics(eval_preds):
    """Custom metrics for evaluating context-aware performance."""
    predictions, labels = eval_preds
    predictions = np.argmax(predictions, axis=-1)
    mask = labels != -100
    labels = labels[mask]
    predictions = predictions[mask]
    accuracy = (predictions == labels).mean()
    return {"accuracy": accuracy}

def main():
    parser = argparse.ArgumentParser(description="Improved training process for context handling")
    parser.add_argument("--input_file", type=str, default="improved.nl", help="Path to input file")
    parser.add_argument("--output_file", type=str, default="improved.cm", help="Path to output file")
    parser.add_argument("--model_id", type=str, default="meta-llama/Llama-3.2-3B", help="Hugging Face model ID")
    parser.add_argument("--output_dir", type=str, default=DEFAULT_OUTPUT_DIR, help="Output directory (Google Drive folder if in Colab)")
    parser.add_argument("--batch_size", type=int, default=10, help="Batch size for training")
    parser.add_argument("--learning_rate", type=float, default=3e-4, help="Learning rate")
    parser.add_argument("--num_epochs", type=int, default=2, help="Number of training epochs")
    parser.add_argument("--warmup_ratio", type=float, default=0.1, help="Warmup ratio")
    parser.add_argument("--max_length", type=int, default=512, help="Max length for tokenization")
    parser.add_argument("--seed", type=int, default=42, help="Random seed")
    parser.add_argument("--use_wandb", action="store_true", help="Use Weights & Biases for tracking")
    parser.add_argument("--no_gradient_checkpointing", action="store_true", help="Disable gradient checkpointing")
    parser.add_argument("--force_cpu", action="store_true", help="Force the use of CPU even if a GPU is available")
    
    args, _ = parser.parse_known_args()
    
    seed_everything(args.seed)
    
    if args.use_wandb:
        wandb.init(project="llama2-terminal-commands", name="context-improved")
    
    if args.force_cpu:
        device = "cpu"
    else:
        device = "cuda" if torch.cuda.is_available() else "cpu"
    print(f"Using device: {device}")
    
    print("Loading data...")
    train_dataset, val_dataset = load_data(args.input_file, args.output_file)
    print(f"Loaded {len(train_dataset)} training examples and {len(val_dataset)} validation examples")
    
    print(f"Loading model {args.model_id}...")
    model, tokenizer = load_model(args.model_id, device)
    
    print("Tokenizing datasets...")
    tokenized_train = train_dataset.map(
        lambda examples: tokenize_function(examples, tokenizer, args.max_length),
        batched=True,
        remove_columns=train_dataset.column_names
    )
    tokenized_val = val_dataset.map(
        lambda examples: tokenize_function(examples, tokenizer, args.max_length),
        batched=True,
        remove_columns=val_dataset.column_names
    )
    
    data_collator = DataCollatorForLanguageModeling(
        tokenizer=tokenizer,
        mlm=False
    )
    
    training_args = TrainingArguments(
        output_dir=args.output_dir,
        per_device_train_batch_size=args.batch_size,
        per_device_eval_batch_size=args.batch_size,
        evaluation_strategy="steps",
        eval_steps=500,
        logging_steps=50,
        gradient_accumulation_steps=4,
        num_train_epochs=args.num_epochs,
        weight_decay=0.01,
        warmup_ratio=args.warmup_ratio,
        lr_scheduler_type="cosine",
        learning_rate=args.learning_rate,
        save_steps=200,
        save_total_limit=3,
        load_best_model_at_end=True,
        metric_for_best_model="eval_loss",
        greater_is_better=False,
        push_to_hub=False,
        report_to="wandb" if args.use_wandb else "none",
        gradient_checkpointing=not args.no_gradient_checkpointing,
        fp16=device == "cuda",
        ddp_find_unused_parameters=False,
        dataloader_drop_last=True,
        optim="adamw_torch",
        remove_unused_columns=False,
    )
    
    from transformers.trainer import Trainer as BaseTrainer
    original_move_model = BaseTrainer._move_model_to_device
    def safe_move_model(self, model, device):
        if any(p.device.type == "meta" for p in model.parameters()):
            print("Detected meta tensors, using to_empty() to move model")
            return model.to_empty(device=device)
        return original_move_model(self, model, device)
    BaseTrainer._move_model_to_device = safe_move_model
    
    trainer = Trainer(
        model=model,
        args=training_args,
        train_dataset=tokenized_train,
        eval_dataset=tokenized_val,
        tokenizer=tokenizer,
        data_collator=data_collator,
        callbacks=[EarlyStoppingCallback(early_stopping_patience=3)],
    )
    
    print("Starting training...")
    trainer.train()
    
    print(f"Saving model to {args.output_dir}")
    trainer.save_model(args.output_dir)
    tokenizer.save_pretrained(args.output_dir)
    
    print("Training complete!")
    
    if args.use_wandb:
        wandb.finish()

if __name__ == "__main__":
    main()
