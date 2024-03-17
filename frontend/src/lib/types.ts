interface Metadata {
    creationTimestamp?: string;
    deletionGracePeriodSeconds?: number;
    deletionTimestamp?: string;
    generateName?: string;
    labels?: {
      [key: string]: string;
    };
    managedFields?: {
      apiVersion?: string;
      fieldsType?: string;
      fieldsV1?: {
        [key: string]: any; // eslint-disable-line @typescript-eslint/no-explicit-any
      };
      manager?: string;
      operation?: string;
      time?: string;
    }[];
    name?: string;
    namespace?: string;
    ownerReferences?: {
      apiVersion?: string;
      blockOwnerDeletion?: boolean;
      controller?: boolean;
      kind?: string;
      name?: string;
      uid?: string;
    }[];
    resourceVersion?: string;
    uid?: string;
  }
  
  interface Container {
    args?: string[];
    env?: {
      name?: string;
      value?: string;
    }[];
    image?: string;
    imagePullPolicy?: string;
    name?: string;
    ports?: {
      containerPort?: number;
      name?: string;
      protocol?: string;
    }[];
    resources?: {
      limits?: {
        cpu?: string;
        memory?: string;
      };
      requests?: {
        cpu?: string;
        memory?: string;
      };
    };
    terminationMessagePath?: string;
    terminationMessagePolicy?: string;
    volumeMounts?: {
      mountPath?: string;
      name?: string;
      readOnly?: boolean;
    }[];
  }
  
  interface ProjectedSource {
    serviceAccountToken?: {
      audience?: string;
      expirationSeconds?: number;
      path?: string;
    };
    configMap?: {
      name?: string;
      items?: {
        key?: string;
        path?: string;
      }[];
    };
    downwardAPI?: {
      items?: {
        fieldRef?: {
          apiVersion?: string;
          fieldPath?: string;
        };
        path?: string;
      }[];
    };
  }
  
  interface Volume {
    name?: string;
    projected?: {
      defaultMode?: number;
      sources?: ProjectedSource[];
    };
  }
  
  interface StatusCondition {
    lastTransitionTime?: string;
    status?: string;
    type?: string;
  }
  
  interface ContainerStatus {
    containerID?: string;
    image?: string;
    imageID?: string;
    lastState?: any; // eslint-disable-line @typescript-eslint/no-explicit-any
    name?: string;
    ready?: boolean;
    restartCount?: number;
    started?: boolean;
    state?: {
      running?: {
        startedAt?: string;
      };
    };
  }
  
  interface Status {
    conditions?: StatusCondition[];
    containerStatuses?: ContainerStatus[];
    hostIP?: string;
    hostIPs?: { ip?: string }[];
    phase?: string;
    podIP?: string;
    podIPs?: { ip?: string }[];
    qosClass?: string;
    startTime?: string;
  }
  
  interface Pod {
    rowClass?: string;
    apiVersion?: string;
    kind: string;
    metadata: Metadata;
    spec: {
      containers?: Container[];
      dnsPolicy?: string;
      enableServiceLinks?: boolean;
      nodeName?: string;
      preemptionPolicy?: string;
      priority?: number;
      restartPolicy?: string;
      schedulerName?: string;
      securityContext?: any; // eslint-disable-line @typescript-eslint/no-explicit-any
      serviceAccount?: string;
      serviceAccountName?: string;
      terminationGracePeriodSeconds?: number;
      tolerations?: {
        effect?: string;
        key?: string;
        operator?: string;
        tolerationSeconds?: number;
      }[];
      volumes?: Volume[];
    };
    status: Status;
  }
  
  export type { Pod, Status, ContainerStatus, StatusCondition };