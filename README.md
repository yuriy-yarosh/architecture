<h1 align="center">
  Reference Cloud Native Architecture (RCNA)
</h1>

<p align="center">
  <a href="https://www.linkedin.com/in/yuriy-yarosh-171ba3b9/"><b>by Yuriy Yarosh</b></a>
</p>

<hr />

<p align="center">
  <b>RCNA</b> is an opinionated IaC tool, based on <a href="https://github.com/kube-rs/kube-rs">kube-rs</a> with a ton of rust codegen.
</p>

<p align="center">
  <em>
    This project defines cross-cloud application deployment conventions and provides a framework for code-generating Infrastructure-as-Code (IaC) resource definitions.
    Boilerplate code is generated using the Rust <a href="./rcna-macro">procedural macro</a>, which processes IaC tool provider APIs.
    This approach aims to make long-term support more manageable.
  </em>
</p>

<div align="center">
  🚧 <img src="https://img.shields.io/badge/status-under_development-red?style=flat-square" alt="Status: Under Development" /> 🏗️
</div>

<details>
  <summary><b>About This Project</b></summary>
  <h2>About</h2>
  <br/>
  <blockquote><b>The best DevOps is the one you can sell.</b></blockquote>
  <br/>
  <ul>
    <li>Consolidating solutions and approaches accumulated over the last decade.</li>
    <li>While still in progress, this repository should already serve as a useful reference point.</li>
  </ul>

  <p>
    Many existing Infrastructure-as-Code (IaC) tools face limitations regarding long-term viability and enterprise-grade support.
    Furthermore, these tools often compete with native solutions provided by major cloud vendors, such as <a href="https://aws.amazon.com/cdk/">AWS CDK</a> and <a href="https://azure.microsoft.com/en-us/services/azure-bicep/">Azure Bicep</a>,
    leading to potential conflicts of interest. Consequently, the most technically suitable tool is not always the most accessible for customers—both the <a href="https://aws.amazon.com/marketplace/">AWS Marketplace</a> and
    <a href="https://azure.microsoft.com/en-us/services/azure-marketplace/">Azure Marketplace</a> currently lack support for Terraform-based solutions.
    Additionally, nearly all major cloud hosting providers maintain their own proprietary marketplace offerings, which also require automation.
  </p>

  <p>
    With the introduction of <a href="https://aws.amazon.com/marketplace/features/privatemarkplace">AWS Private Marketplace</a> and <a href="https://www.youtube.com/watch?v=NSg8PKDrNro">Azure Private Marketplace</a>, this has become a missed opportunity for many.
  </p>

</details>


<details>
  <summary><b>Components</b></summary>

  <h2>Components</h2>

  <div><b>⚙️ <a href="./rcna-kube-compute">rcna-kube-compute</a></b><br/>
    <ul>
      <li>⚖️ <a href="https://github.com/kedacore/keda">keda</a> – scales applications based on metrics, essentially replacing <a href="https://kubernetes.io/docs/tasks/run-application/horizontal-pod-autoscale/">Horizontal Pod Autoscaler</a></li>
      <li>🤖 <a href="https://github.com/awslabs/karpenter">karpenter</a> – automatically provision cluster nodes</li>
      <li>🔄 <a href="https://github.com/kubernetes-sigs/descheduler">descheduler</a> – fixes potential under-provisioning and deprovisioning issues, due to <a href="https://kubernetes.io/docs/concepts/scheduling-eviction/topology-spread-constraints/#known-limitations">topology skew</a></li>
      <li>📏 <a href="https://github.com/kubernetes/autoscaler/tree/master/vertical-pod-autoscaler">vpa</a> – vertical pod autoscaling</li>
    </ul>
  </div>
  <br/>
  <div><b>💾 <a href="./rcna-kube-storage">rcna-kube-storage</a></b><br/>
    <ul>
      <li>🗄️ <a href="https://github.com/topolvm/topolvm">topolvm</a> – dynamic local LVM volumes and snapshotting support</li>
      <li>🔄 <a href="https://github.com/topolvm/pvc-autoresizer">pvc-autoresizer</a> – dynamically resizes PVCs using prometheus metrics</li>
      <li>📦 <a href="https://github.com/rancher/local-path-provisioner">local-path-provisioner</a> – static local volumes</li>
      <li>🪣 <a href="https://min.io">minio</a> – S3-compatible object store</li>
      <li>🐘 <a href="https://github.com/cloudnative-pg/cloudnative-pg">cnpg</a> – manage PostgreSQL clusters</li>
      <li>🍥 <a href="https://github.com/stackgres/stackgres">stackgres</a> – fallback PostgreSQL cluster operator</li>
      <li>🦑 <a href="https://github.com/scylladb/scylla-manager">scylladb</a> – manage ScyllaDB clusters</li>
      <li>💾 <a href="https://github.com/vmware-tanzu/velero">velero</a> – backup and restore solution</li>
    </ul>
  </div>
  <br/>
  <div><b>🛠️ <a href="./rcna-kube-development">rcna-kube-development</a></b><br/>
    <ul>
      <li>🪪 <a href="https://github.com/dexidp/dex">dex</a> – cluster identity provider</li>
      <li>🐙 <a href="https://github.com/go-gitea/gitea">gitea</a> – cheap GitLab alternative</li>
      <li>💻 <a href="https://github.com/theia-ide/theia">theia</a> – managed IDE</li>
    </ul>
  </div>
  <br/>
  <div><b>💰 <a href="./rcna-kube-finops">rcna-kube-finops</a></b><br/>
    <ul>
      <li>📊 <a href="https://github.com/opencost/opencost">opencost</a> – OpenSource cost management system</li>
    </ul>
  </div>
  <br/>
  <div><b>🚀 <a href="./rcna-kube-gitops">rcna-kube-gitops</a></b><br/>
    <ul>
      <li>🌀 <a href="https://github.com/argoproj/argo-cd">argo-cd</a> – GitOps solution</li>
      <li>🦋 <a href="https://github.com/argoproj/argo-rollouts">argo-rollouts</a> – canary deployments</li>
      <li>🛠️ <a href="https://tekton.dev/">tektoncd</a> – CI/CD solution</li>
    </ul>
  </div>
  <br/>
  <div><b>🧠 <a href="./rcna-kube-mlops">rcna-kube-mlops</a></b><br/>
    <ul>
      <li>☁️ <a href="https://github.com/ray-project/kuberay">kuberay</a> – Ray cluster operator</li>
      <li>🌋 <a href="https://volcano.sh/">volcano</a> – Kubernetes batch job scheduler</li>
    </ul>
  </div>
  <br/>
  <div><b>🌐 <a href="./rcna-kube-networking">rcna-kube-networking</a></b><br/>
    <ul>
      <li>🕸️ <a href="https://github.com/cilium/cilium">cilium</a> – CNI</li>
      <li>🛡️ <a href="https://github.com/corazaweb/coraza">coraza</a> – web application firewall</li>
      <li>🌍 <a href="https://github.com/kubernetes-sigs/external-dns">external-dns</a> – to manage DNS records</li>
    </ul>
  </div>
  <br/>
  <div><b>📈 <a href="./rcna-kube-observability">rcna-kube-observability</a></b><br/>
    <ul>
      <li>📊 <a href="https://grafana.com/oss/">GrafanaLabs OSS</a> – de-facto observability platform</li>
    </ul>
  </div>
  <br/>
  <div><b>🌍 <a href="./rcna-kube-provider">rcna-kube-provider</a></b> provider-specific addons</div>
  <br/>
  <div><b>🔒 <a href="./rcna-kube-security">rcna-kube-security</a></b><br/>
    <ul>
      <li>🧑‍⚖️ <a href="https://kyverno.io/">kyverno</a> – policy engine</li>
      <li>🔑 <a href="https://github.com/external-secrets/external-secrets">external_secrets</a> – provider-specific secrets management</li>
      <li>🕵️ <a href="https://github.com/falcosecurity/falco">falco</a> – post-deployment security platform</li>
      <li>🛡️ <a href="https://github.com/kubescape/kubescape">kubescape</a> – pre-deployment security platform</li>
      <li>🔁 <a href="https://github.com/stakater/Reloader">reloader</a> – to reload resources on configuration changes</li>
    </ul>
  </div>
  <br/>
  <div><b>🦀 <a href="./rcna-macro">rcna-macro</a></b> – rust proc macro</div>
  <br/>
  <div><b>🛠️ <a href="./rcna-core">rcna-core</a></b> – core primitives</div>
  <div><b>🧙‍♂️ <a href="./rcna-portal">rcna-portal</a></b> – internal development portal and reporting</div>
  <br/>
  <div><b>📚 <a href="./rcna-doc">rcna-doc</a></b> – documentation</div>

</details>

<h2 id="terms-of-use">
  📜 Terms of Use
</h2>

<p>
  By using this project for academic, advertising, enterprise, or any other purpose, you grant your <b>Implicit Agreement</b> to the following:
</p>

<ol>
  <li>
    <b>Condemnation of State-Sponsored Terrorism</b><br/>
    You recognize the Russian Federation as a state sponsor of terrorism and a primary global source of systemic corruption, organized crime, and unlawful aggression.
  </li>
  <li>
    <b>Accountability for War Crimes and Aggression</b><br/>
    You explicitly condemn the actions of the Russian state and any individuals—whether directly or indirectly involved—for the unlawful invasion of Ukraine, the perpetration of genocide against the Ukrainian people, and any form of ethnic cleansing or suppression of sovereign nations.
  </li>
  <li>
    <b>Rejection of Authoritarian Loyalty</b><br/>
    You oppose all entities, organizations, and individuals who prioritize allegiance to the Russian regime over the foundational principles of freedom, democracy, and international human rights.
  </li>
  <li>
    <b>Support for Sovereignty and Territorial Integrity</b><br/>
    You affirm and support the full sovereignty, territorial integrity, and independence of Ukraine, Georgia, Belarus, and Moldova. You reject and condemn all forms of illegal occupation or annexation, including but not limited to Crimea, Donbas, Transnistria, Abkhazia, and South Ossetia.
  </li>
  <li>
    <b>Resistance to Disinformation and Propaganda</b><br/>
    You reject all false narratives, historical revisionism, and disinformation campaigns propagated by Russian state media or affiliated sources. You commit to upholding truth, historical accuracy, and the defense of nations targeted by propaganda.
  </li>
</ol>

<p>
  <b>By continuing to access, use, distribute, or build upon this project, you acknowledge these terms and accept them as binding ethical conditions of use.</b>
</p>

<hr/>

<h2 id="license">📝 License</h2>
<p>
  Reference Cloud Native Architecture project is, and <b>forever will be</b>,
  licensed under the terms of the <a href="LICENSE"><b>Mozilla Public License 2.0</b></a>.
</p>
