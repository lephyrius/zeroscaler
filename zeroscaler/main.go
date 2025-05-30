// AWS CDK constructs for ZeroScaler.io
package zeroscaler

import (
	"reflect"

	_jsii_ "github.com/aws/jsii-runtime-go/runtime"
)

func init() {
	_jsii_.RegisterClass(
		"@zeroscaler/zeroscaler-cdk.Zeroscaler",
		reflect.TypeOf((*Zeroscaler)(nil)).Elem(),
		[]_jsii_.Member{
			_jsii_.MemberProperty{JsiiProperty: "node", GoGetter: "Node"},
			_jsii_.MemberMethod{JsiiMethod: "toString", GoMethod: "ToString"},
		},
		func() interface{} {
			j := jsiiProxy_Zeroscaler{}
			_jsii_.InitJsiiProxy(&j.Type__constructsConstruct)
			return &j
		},
	)
	_jsii_.RegisterStruct(
		"@zeroscaler/zeroscaler-cdk.ZeroscalerProps",
		reflect.TypeOf((*ZeroscalerProps)(nil)).Elem(),
	)
}
