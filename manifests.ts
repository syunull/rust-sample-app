import { App, YamlOutputType } from "cdk8s";
import { Chart, ChartProps } from "cdk8s";
import { Construct } from "constructs";
import { Namespace } from "cdk8s-plus-24";
import { KubeResourceQuota } from "./imports/k8s";

export interface ProjectProps extends ChartProps {
  name: string;
  namespace: string;
  namespaceLabels: any;
}

export class Project extends Chart {
  readonly projectNamespace: Namespace;
  readonly resourceQuota: KubeResourceQuota;

  constructor(scope: Construct, id: string, props: ProjectProps) {
    super(scope, id, props);

    this.projectNamespace = new Namespace(this, "namespace", {
      metadata: { name: props.name, labels: props.namespaceLabels },
    });

    this.resourceQuota = new KubeResourceQuota(this, "resourcequota", {
      metadata: { name: props.name },
      spec: {
        hard: {
          pods: { value: "12" },
          "limits.cpu": { value: "5" },
          "limits.memory": { value: "6Gi" },
        },
      },
    });
  }
}

const app = new App({
  yamlOutputType: YamlOutputType.FILE_PER_APP,
});

const project = new Project(app, "project", {
  name: "rust-app",
  namespace: "sample-app",
  namespaceLabels: {
    "linkerd.io/inject": "enable",
  },
});

app.synth();
