# Zeroscaler CDK TypeScript Construct Library project

[![npm version](https://badge.fury.io/js/zeroscaler.svg)](https://badge.fury.io/js/zeroscaler)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.8+-blue.svg)](https://www.typescriptlang.org/)
[![AWS CDK](https://img.shields.io/badge/AWS%20CDK-2.0+-orange.svg)](https://aws.amazon.com/cdk/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](http://makeapullrequest.com)

Construct (`Zeroscaler`) which contains and deploys a lambda that boots up your Fargate application when there is an incoming request. It also monitors

The construct defines an interface (`ZeroscalerProps`) to configure the visibility timeout of the queue.

## Useful commands

* `npm run build`   compile typescript to js
* `npm run watch`   watch for changes and compile
* `npm run test`    perform the jest unit tests

## Diagram of the Construct

```mermaid
flowchart TD
    A["Client"] L_A_B_0@-- HTTP Request --> B["Zeroscaler"]
    B -. Register Target .-> C["ELB Target Group"]
    B L_B_A_0@-- Serve HTML with refresh --> A
    C -- Health Check --> D["ECS Fargate"]
    D -- Boot --> E["Your Fargate Application"]
    A L_A_E_0@-- Refresh when booted --> E
    E L_E_A_0@-- Response --> A
    linkStyle 0 stroke:#00C853,fill:none
    linkStyle 1 stroke:#AA00FF,fill:none
    linkStyle 2 stroke:#00C853,fill:none
    linkStyle 3 stroke:#AA00FF,fill:none
    linkStyle 4 stroke:#AA00FF,fill:none
    linkStyle 5 stroke:#2962FF,fill:none
    linkStyle 6 stroke:#2962FF,fill:none
    L_A_B_0@{ animation: fast } 
    L_B_A_0@{ animation: fast } 
    L_A_E_0@{ animation: fast } 
    L_E_A_0@{ animation: fast }
```