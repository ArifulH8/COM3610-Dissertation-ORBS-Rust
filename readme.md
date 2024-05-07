# ORBS-Rust

This project was made for COM3610: Dissertation Project.  
It is an implementation of Observation Based Slicing using test suite as slice validation in Rust.

## Set Up

This project uses Rust v1.76.0.  
Once the repository is cloned, change directory to it.  
Install the orbs slicer using the following command.  
```shell
cargo install --path .
```

The slicer can be invoked using the command `orbs_slicer`.  
The following message will be shown when using the help command.
```shell
orbs_slicer -h
Usage: orbs_slicer.exe [OPTIONS] --folder-path <FOLDER_PATH> --test-command <TEST_COMMAND>      
Options:
      --folder-path <FOLDER_PATH>    Folder path of the project to be sliced
      --file-path <FILE_PATH>        File path of the file to be sliced
      --test-command <TEST_COMMAND>  Test command to be used for slicing
      --test-args <TEST_ARGS>        Test args given to the test command
      --skip-files <SKIP_FILES>      Names of files to skip when slicing multiple files
  -d, --del-win <DEL_WIN>            Number of deletion windows (>0) [default: 3]
  -v, --verbose                      Set true when you need detailed output
  -h, --help                         Print help
  -V, --version                      Print version
```

## Examples
This section assume that each example has been fully set up.

### Javascript
You can run the javascript example using the following command.  

```shell
orbs_slicer --folder-path ./examples/ex_js/src --file-path ./examples/ex_js/src/index.js --test-command "npm run" --test-args "test" --skip-files "test"
```

```shell
orbs_slicer --folder-path ./examples/ex_js2/src --file-path ./examples/ex_js2/src/index.js --test-command "npm run" --test-args "test" --skip-files "test"
```

### Python

```shell
orbs_slicer --folder-path ./examples/ex_py/src --file-path ./examples/ex_py/src/main.py --test-command "python" --test-args "tests.py" --skip-files "test"
```

### Ruby

```shell
orbs_slicer --folder-path ./examples/ex_rb/src --file-path ./examples/ex_rb/src/main.rb --test-command "rspec" --test-args "main_spec.rb" --skip-files "spec" --skip-files "Gemfile"
```

### Other projects
The slicer can also slice other projects given there is a suitable test suite.
Ensure that configuration, test files and other files that do not need to be sliced is added as a `--skip-files` argument.
