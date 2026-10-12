
export type ImageSummary = {
  Id: string;
  RepoTags: string[];
  Size: number;
  Created: number;
};

export type ImageInspect = {
  Id: string;
  RepoTags?: string[];
  RepoDigests?: string[];
  Parent?: string;
  Comment?: string;
  Created?: string;
  DockerVersion?: string;
  Author?: string;
  Architecture?: string;
  Variant?: string;
  Os?: string;
  Size?: number;
  VirtualSize?: number;
  GraphDriver?: { Name?: string; Data?: Record<string, string> };
  RootFS?: { Type?: string; Layers?: string[] };
  Config?: {
    Hostname?: string;
    User?: string;
    Env?: string[];
    Cmd?: string[];
    Image?: string;
    WorkingDir?: string;
    Entrypoint?: string[];
    Labels?: Record<string, string>;
  };
};

export type ImageHistoryItem = {
  Id: string;
  Created?: number;
  CreatedBy?: string;
  Tags?: string[] | null;
  Size?: number;
  Comment?: string;
};

export type FsNode = {
  name: string;
  path: string;
  type: "dir" | "file" | "symlink" | "other";
  size: number;
  link?: string;
  children?: FsNode[];
};

export type ImageFs = {
  root: FsNode;
  file_count: number;
  dir_count: number;
  truncated: boolean;
};
