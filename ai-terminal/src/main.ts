import { bootstrapApplication } from "@angular/platform-browser";
import { appConfig } from "./app/app.config";
import { AppComponent } from "./app/app.component";

console.log('🚀 Starting AI Terminal...');
console.log('Environment:', {
  userAgent: navigator.userAgent,
  location: window.location.href,
  timestamp: new Date().toISOString()
});

bootstrapApplication(AppComponent, appConfig)
  .then(() => {
    console.log('✅ AI Terminal started successfully!');
  })
  .catch((err) => {
    console.error('❌ Failed to start AI Terminal:', err);
    // Show error on page if Angular failed to start
    document.body.innerHTML = `
      <div style="
        background: #1e1e1e;
        color: #ff6b6b;
        font-family: monospace;
        padding: 20px;
        height: 100vh;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
      ">
        <h1>AI Terminal - Startup Error</h1>
        <p>Failed to initialize the application:</p>
        <pre style="background: #2d2d2d; padding: 15px; border-radius: 5px; max-width: 80%; overflow: auto;">${err}</pre>
        <p>Please check the browser console for more details.</p>
      </div>
    `;
  });
