# Ex PY 1

This example made for the slicer.  
Uses ruby version v2.7.2p137 and bundler version v2.4.22.

## Set Up
```shell
bundle install
```

## Run
```shell
ruby .\src\main.rb
```

## Test
```shell
rspec .\src\main_spec.rb
```

### Other projects
The slicer can also slice other projects given there is a suitable test suite.
Ensure that configuration, test files and other files that do not need to be sliced is added as a `--skip-files` argument.
