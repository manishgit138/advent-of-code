from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="advent_of_code_rs_python",
    version="0.0.1",
    rust_extensions=[
        RustExtension("advent_of_code_rs_python", "Cargo.toml", binding=Binding.PyO3)
    ],
    test_suite="tests",
    zip_safe=False,
)