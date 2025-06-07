# RCNA Core

Provides core RCNA abstractions for generating Terraform modules, and Kubernetes YAML.
Designed to complement [reference deployment modules](https://github.com/yuriy-yarosh?tab=repositories&q=terraform-reference-&type=&language=&sort=name).

RCNA Core does not implement missing services or APIs, nor does it introduce additional abstractions beyond the application deployment layer. 
If a service or API is unavailable or unimplemented by the target hosting provider, it is either omitted or, when possible, substituted with the corresponding self-hosted Kubernetes service.

## Features

 - Abstracts both stateful and stateless application deployment targeting reference Operators
 - Ensures auto-scaling according to the current viable well-architected practices
 - Ensures application security with least privilege access and proper security audit
 - Follows the existing organization management conventions
 - Establishes application observability
 
There are some plans regarding **OpenShift** compatibility, but it's not a priority at the moment.

## Usage

## Terms of Use

Shares the same terms of use as the [Reference Cloud Native Architecture](../README.md#terms-of-use) project.

## License

Reference Cloud Native Architecture Core, is licensed under the terms of [Mozilla Public License 2.0](../LICENSE).
