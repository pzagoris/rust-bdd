# API Testing using cucumber-rs

## Prerequisites

Define the following environment variables in the Dockerfile.

```
ENV ENV_EXECUTION= [config-prod.yaml| config-test.yaml | config-dev.yaml]
ENV API_KEY= "..."
ENV API_SECRET= "..."
```

In the 3 yaml files are stored test environment related parameters. Defining the `ENV_EXECUTION` variable you specify in which environment the tests would be executed. All the 3 files should have the fields with different values.

## Test locally

After defining the above variables you can execute the tests locally by running:

```
cargo test --test test_runner
```

## Docker Build

In order to build the docker file and produce the container image,  run the command

```
docker build  --build-arg ENV_EXECUTION=[ENV_EXECUTION]  --build-arg API_KEY=[API_KEY]  --build-arg API_SECRET=[API_SECRET]  -t api-test .
```

## Container Test ENV_EXECUTION

For executing the tests inside the container run the command

```
docker run --rm api-test
```
