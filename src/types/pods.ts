import { ContainerSummary } from "./containers";
import { NetworkSummary } from "./networks";

export type PodSummary = {
  Cgroup: string;
  Containers: ContainerSummary[];
  Created: Date;
  Id: string;
  InfraId: string;
  Labels: string;
  Name: string;
  Namespace: string;
  Networks: NetworkSummary[];
  Status: string;
}
