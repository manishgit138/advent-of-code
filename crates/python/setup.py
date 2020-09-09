from os import path

from setuptools import setup
from setuptools_rust import Binding, RustExtension

# Read the contents of the README file:
this_directory = path.abspath(path.dirname(__file__))
with open(path.join(this_directory, "README.md"), encoding="utf-8") as f:
    long_description = f.read()

setup(
    name="advent_of_code_rs_python",
    long_description=long_description,
    long_description_content_type='text/markdown',
    version="2019.12.37",
    rust_extensions=[
        RustExtension("advent_of_code_rs_python", "Cargo.toml", binding=Binding.PyO3)
    ],
    test_suite="tests",
    zip_safe=False,
)
