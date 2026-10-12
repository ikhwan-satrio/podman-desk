
export type ContainerSummary = {
  Id: string;
  Names: string[];
  State?: string;
  Status: string;
  Image: string;
};

export type ContainerState = {
  Status?: string;
  Running?: boolean;
  Paused?: boolean;
  Restarting?: boolean;
  OOMKilled?: boolean;
  Dead?: boolean;
  Pid?: number;
  ExitCode?: number;
  Error?: string;
  StartedAt?: string;
  FinishedAt?: string;
};

export type ContainerPort = {
  IP?: string;
  PrivatePort?: number;
  PublicPort?: number;
  Type?: string;
};

export type ContainerMount = {
  Type?: string;
  Name?: string;
  Source?: string;
  Destination?: string;
  Driver?: string;
  Mode?: string;
  RW?: boolean;
  Propagation?: string;
};

export type ContainerNetwork = {
  IPAMConfig?: unknown;
  Links?: string[];
  Aliases?: string[];
  NetworkID?: string;
  EndpointID?: string;
  Gateway?: string;
  IPAddress?: string;
  IPPrefixLen?: number;
  IPv6Gateway?: string;
  GlobalIPv6Address?: string;
  GlobalIPv6PrefixLen?: number;
  MacAddress?: string;
  DriverOpts?: Record<string, string>;
  GwPriority?: number;
  NetworkNames?: string[];
};

export type ContainerInspect = {
  Id: string;
  Name?: string;
  Created?: string;
  Path?: string;
  Args?: string[];
  Image?: string;
  ResolvedImageName?: string;
  RestartCount?: number;
  Driver?: string;
  Platform?: string;
  State?: ContainerState;
  Config?: {
    Hostname?: string;
    Domainname?: string;
    User?: string;
    AttachStdin?: boolean;
    AttachStdout?: boolean;
    AttachStderr?: boolean;
    ExposedPorts?: Record<string, unknown>;
    Tty?: boolean;
    OpenStdin?: boolean;
    StdinOnce?: boolean;
    Env?: string[];
    Cmd?: string[];
    Image?: string;
    Volumes?: Record<string, unknown>;
    WorkingDir?: string;
    Entrypoint?: string[];
    OnBuild?: unknown[];
    Labels?: Record<string, string>;
  };
  HostConfig?: {
    NetworkMode?: string;
    RestartPolicy?: { Name?: string; MaximumRetryCount?: number };
  };
  NetworkSettings?: {
    Bridge?: string;
    SandboxID?: string;
    Ports?: Record<string, ContainerPort[] | null>;
    Networks?: Record<string, ContainerNetwork>;
  };
  Mounts?: ContainerMount[];
};
