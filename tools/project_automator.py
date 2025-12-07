#!/usr/bin/env python3
import subprocess
import sys
import os

def check_dependencies():
    print("🔍 Checking environment...")
    deps = ["node", "npm", "cargo", "adb", "fastboot"]
    missing = []
    for dep in deps:
        if subprocess.call(f"which {dep}", shell=True, stdout=subprocess.DEVNULL) != 0:
            missing.append(dep)
    
    if missing:
        print(f"❌ Missing dependencies: {', '.join(missing)}")
        return False
    print("✅ All core dependencies found.")
    return True

def build_frontend():
    print("🎨 Building Vue Frontend...")
    return subprocess.call("npm run build", shell=True) == 0

def build_backend():
    print("🦀 Building Rust Backend...")
    return subprocess.call("cargo build --release", cwd="src-tauri", shell=True) == 0

def main():
    if not check_dependencies():
        sys.exit(1)
        
    if len(sys.argv) > 1 and sys.argv[1] == "full":
        if build_frontend() and build_backend():
            print("🚀 Full build complete!")
        else:
            print("💥 Build failed.")
    else:
        # Default to just frontend for quick iteration
        if build_frontend():
            print("✨ Frontend updated.")

if __name__ == "__main__":
    main()
