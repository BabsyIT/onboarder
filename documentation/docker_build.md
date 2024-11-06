# Docker build

## Build

You should comply to the following naming convention:

image_name = "ghcr.io/babsyit/onboarder"

Never push an image that you built locally. Always use the CI/CD pipeline to build and push the image.

```bash
docker build -t {{image_name}}:{{version}} .
```

## Publish

This will publish the image to the registry.

```bash
just push
```

## Automated build

1. We use Semantic Versioning. The version is determined by the git commit message.

```bash
git commit -m "feat: add new feature"
```

> Will increase the second number: x.1.x

```bash
git commit -m "fix: add new fix"
```

> Will increase the last number: x.x.1

```bash
git commit -m "feat!: add new breaking feature"
```

> Will increase the first number: 1.x.x

2. Then it will create a pr, if you merge that release pr it will automatically build and push the image to the registry.
