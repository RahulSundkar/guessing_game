import os
import subprocess
import git

git.Git("RahulSundkar/").clone("https://github.com/RahulSundkar/guessing_game.git")

print("\n\n")
try:
    git_version = subprocess.check_output(['git', '--version'])
    print(f"Git is installed. Version: {git_version.decode().strip()}")
except subprocess.CalledProcessError:
    print("Git is not installed.")
    os._exit(0)

try:
    rust_version = subprocess.check_output(['rustc', '--version'])
    print(f"Rust is installed. Version: {rust_version.decode().strip()}")
except subprocess.CalledProcessError:
    print("Rust is not installed.")
    os._exit(0)

try:
    cargo_version = subprocess.check_output(['cargo', '--version'])
    print(f"Cargo is installed. Version: {cargo_version.decode().strip()}")
except subprocess.CalledProcessError:
    print("Cargo is not installed.")
    os._exit(0)


subprocess.run(['cargo', 'build'], cwd='guessing_game')
subprocess.run(['./guessing_game/target/debug/guessing_game.exe'])
