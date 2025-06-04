<h2 align="center">
  RCNA Deployment Modules
</h2>

<hr />

<p align="center">
  <b>RCNA Deployment</b> provides ready-to-use, reference Infrastructure-as-Code (IaC) modules for major cloud providers and popular IaC frameworks.
</p>

<h2>About These Modules</h2>
<br/>
<ul>
  <li>Reference implementations for deploying RCNA across different cloud providers.</li>
  <li>Supports <a href="https://www.terraform.io">Terraform</a>, <a href="https://aws.amazon.com/cdk/">AWS CDK</a>, <a href="https://azure.microsoft.com/en-us/services/azure-bicep/">Azure Bicep</a>, and <a href="https://www.pulumi.com/">Pulumi</a>.</li>
  <li>Designed for both marketplace and conventional organizational management use cases.</li>
</ul>

<h3>Terraform Modules</h3>
<ul>
  <li><b><a href="./terraform-azure-rcna">Azure</a></b> <em>Azure is common choice for EU deployments.</em></li>
  <li><b><a href="./terraform-aws-rcna">AWS</a></b> <em>AWS is common choice for US deployments.</em></li>
  <li><b><a href="./terraform-gcp-rcna">GCP</a></b> <em>GCP does not fully support native IaC solutions.</em></li>
  <li><b><a href="./terraform-do-rcna">DigitalOcean</a></b> <em>Cost-effective for small-scale deployments.</em></li>
  <li><b><a href="./terraform-hetzner-rcna">Hetzner</a></b> <em>Cost-effective for small-scale deployments in Germany.</em></li>
  <li><b><a href="./terraform-ovh-rcna">OVH</a></b> <em>Cost-effective for small-scale deployments in general. A well <a href="../rcna-doc/img/ovh-burning.png">known meme</a> of cloud migration.</em></li>
  <li><b><a href="./terraform-vultr-rcna">Vultr</a></b> <em>Cost-effective fast storage offerings, and competitive pricing.</em></li>
</ul>

<h3>AWS CDK</h3>
<p>
  <a href="https://aws.amazon.com/cdk/">AWS CDK</a> is a good option for AWS Marketplace deployment with custom Organization <a href="https://docs.aws.amazon.com/prescriptive-guidance/latest/migration-aws-environment/understanding-landing-zones.html">Landing Zone</a>.
</p>

<h3>Azure Bicep</h3>
<p>
  Similarly to <a href="./cdk-aws-rcna">AWS CDK</a>, <a href="https://azure.microsoft.com/en-us/services/azure-bicep/">Azure Bicep</a> is a good option for Azure Marketplace deployments.
</p>

<h3>Pulumi Modules</h3>
<ul>
  <li><b><a href="./pulumi-aws-rcna">AWS</a></b></li>
  <li><b><a href="./pulumi-azure-rcna">Azure</a></b></li>
  <li><b><a href="./pulumi-gcp-cnra">GCP</a></b></li>
  <li><b><a href="./terraform-do-rcna">DigitalOcean</a></b></li>
  <li><b><a href="./terraform-hetzner-rcna">Hetzner</a></b></li>
  <li><b><a href="./terraform-ovh-rcna">OVH</a></b></li>
  <li><b><a href="./terraform-vultr-rcna">Vultr</a></b></li>
</ul>

<h3>Terms of Use</h3>
<p>
  Deployment modules share the same terms of use as the <a href="../README.md#terms-of-use">Reference Cloud Native Architecture</a> project.
</p>

<h3>License</h3>
<p>
  Reference Cloud Native Architecture deployment modules are licensed under the terms of the <a href="../LICENSE">Mozilla Public License 2.0</a>.
</p>
