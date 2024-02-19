import os
import pathlib
import sys

if __name__ == "__main__":
    # Ensure maturin is installed
    os.system("pip install --disable-pip-version-check maturin==1.1.0")

    # Build bindings
    os.system("maturin build --release -m Cargo.toml --interpreter " + sys.executable)

    # Uninstall old version
    os.system("pip uninstall --disable-pip-version-check -y screenton_maker")

    # Install new version
    wheels = pathlib.Path("target/wheels").glob("*.whl")
    latest_wheel = max(wheels, key=lambda p: p.stat().st_mtime)
    os.system(f"pip install --disable-pip-version-check {latest_wheel}")