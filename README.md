# Zeroscaler CDK TypeScript Construct Library project

[![npm version](https://badge.fury.io/js/@zeroscaler%2Fzeroscaler-cdk.svg)](https://badge.fury.io/js/@zeroscaler%2Fzeroscaler-cdk)
[![PyPI version](https://badge.fury.io/py/zeroscaler.svg)](https://badge.fury.io/py/zeroscaler)
[![NuGet version](https://badge.fury.io/nu/ZeroScalerCDK.svg)](https://badge.fury.io/nu/ZeroScalerCDK)
[![Go project version](https://badge.fury.io/go/github.com%2Flephyrius%2Fzeroscaler.svg)](https://pkg.go.dev/github.com/lephyrius/zeroscaler/zeroscaler)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.8+-blue.svg)](https://www.typescriptlang.org/)
[![AWS CDK](https://img.shields.io/badge/AWS%20CDK-2.0+-orange.svg)](https://aws.amazon.com/cdk/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](http://makeapullrequest.com)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

*The Zeroscaler construct* deploys a Lambda function that automatically starts your Fargate application in response to incoming requests. It also handles health monitoring and integrates with an ELB target group.

You can configure the construct using the (`ZeroscalerProps`) interface, which requires a target group ARN and Fargate task ARN. Optionally, you can also specify a custom VPC and ECS cluster.

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

## Installation

```bash
npm install @zeroscaler/zeroscaler-cdk
```

## Usage

```typescript
import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { ZeroScaler } from '@zeroscaler/zeroscaler-cdk';

new Zeroscaler(stack, 'MyZeroscaler', {
    targetGroupArn: 'arn:aws:elasticloadbalancing:...',
    fargateTaskArn: 'arn:aws:ecs:...',
    // Optionally override vpc or cluster
    });
```

## License

MPL-2.0