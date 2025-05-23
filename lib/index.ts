import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as ecs from 'aws-cdk-lib/aws-ecs';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import * as elbv2 from 'aws-cdk-lib/aws-elasticloadbalancingv2';
import * as iam from 'aws-cdk-lib/aws-iam';

export interface ZeroscalerProps {
  readonly vpc: ec2.IVpc;
  readonly cluster: ecs.ICluster;
  readonly targetGroupArn: string;
  readonly fargateTaskArn: string;
  readonly refreshDelay?: string;
}

export class Zeroscaler extends Construct {
  constructor(scope: Construct, id: string, props: ZeroscalerProps) {
    super(scope, id);

    // Lambda function
    const lambdaFn = new lambda.Function(this, 'BootFargateLambda', {
      runtime: lambda.Runtime.PROVIDED_AL2023,
      handler: 'bootstrap',
      code: lambda.Code.fromAsset('../target/lambda/boot_fargate_lambda'), // adjust path as needed
      architecture: lambda.Architecture.ARM_64,
      environment: {
        TARGET_GROUP_ARN: props.targetGroupArn,
        FARGATE_ARN: props.fargateTaskArn,
        REFRESH_DELAY: props.refreshDelay ?? "5"
      },
      timeout: cdk.Duration.seconds(30),
    });

    // Grant Lambda permissions to register targets
    lambdaFn.addToRolePolicy(new iam.PolicyStatement({
      actions: [
        "elasticloadbalancing:RegisterTargets",
        "ecs:DescribeTasks",
        "ec2:DescribeNetworkInterfaces"
      ],
      resources: ["*"], // restrict as needed
    }));

    // ECS Fargate Service (example)
    const taskDef = new ecs.FargateTaskDefinition(this, 'TaskDef');
    // ... add container, etc.

    const service = new ecs.FargateService(this, 'Service', {
      cluster: props.cluster,
      taskDefinition: taskDef,
      desiredCount: 1,
      assignPublicIp: true,
    });

    // Attach to Target Group
    const targetGroup = elbv2.ApplicationTargetGroup.fromTargetGroupAttributes(this, 'TG', {
      targetGroupArn: props.targetGroupArn,
    });
    targetGroup.addTarget(service);

    // Pass Target Group ARN to Fargate container as env var
    taskDef.addContainer('AppContainer', {
      image: ecs.ContainerImage.fromRegistry('amazonlinux'),
      environment: {
        TARGET_GROUP_ARN: props.targetGroupArn,
      },
      memoryLimitMiB: 512,
    });
  }
}
