package zeroscaler

import (
	"github.com/aws/aws-cdk-go/awscdk/v2/awsec2"
	"github.com/aws/aws-cdk-go/awscdk/v2/awsecs"
)

type ZeroscalerProps struct {
	Cluster awsecs.ICluster `field:"required" json:"cluster" yaml:"cluster"`
	FargateTaskArn *string `field:"required" json:"fargateTaskArn" yaml:"fargateTaskArn"`
	TargetGroupArn *string `field:"required" json:"targetGroupArn" yaml:"targetGroupArn"`
	Vpc awsec2.IVpc `field:"required" json:"vpc" yaml:"vpc"`
	RefreshDelay *string `field:"optional" json:"refreshDelay" yaml:"refreshDelay"`
}

