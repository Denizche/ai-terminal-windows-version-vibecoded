class AiTerminal < Formula
  desc "AI-powered terminal with natural language command interface"
  homepage "https://github.com/AiTerminalFoundations/ai-terminal"
  url "https://github.com/AiTerminalFoundations/ai-terminal/releases/download/v0.2.0/ai-terminal.dmg"
  version "0.2.0"
  sha256 "0000000000000000000000000000000000000000000000000000000000000000" # You'll need to update this with the actual SHA256 of your DMG

  def install
    system "hdiutil", "attach", "ai-terminal.dmg"
    system "cp", "-R", "/Volumes/ai-terminal/ai-terminal.app", "#{prefix}/"
    system "hdiutil", "detach", "/Volumes/ai-terminal"
  end

  def caveats
    <<~EOS
      ai-terminal has been installed to:
        #{prefix}/ai-terminal.app
    EOS
  end
end 