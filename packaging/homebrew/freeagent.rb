class Freeagent < Formula
  desc "FreeAgent CLI"
  homepage "https://github.com/amogower/freeagent-cli"
  version "__VERSION__"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/amogower/freeagent-cli/releases/download/v__VERSION__/freeagent-__VERSION__-aarch64-apple-darwin.tar.gz"
      sha256 "__SHA256_MAC_ARM__"
    else
      url "https://github.com/amogower/freeagent-cli/releases/download/v__VERSION__/freeagent-__VERSION__-x86_64-apple-darwin.tar.gz"
      sha256 "__SHA256_MAC_X86__"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/amogower/freeagent-cli/releases/download/v__VERSION__/freeagent-__VERSION__-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "__SHA256_LINUX_ARM__"
    else
      url "https://github.com/amogower/freeagent-cli/releases/download/v__VERSION__/freeagent-__VERSION__-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "__SHA256_LINUX_X86__"
    end
  end

  def install
    bin.install "freeagent"
    generate_completions_from_executable(bin/"freeagent", "completions")
  end

  test do
    assert_match "freeagent", shell_output("#{bin}/freeagent --help")
  end
end
