package zeroscaler

import (
	_jsii_ "github.com/aws/jsii-runtime-go/runtime"
	_init_ "github.com/lephyrius/zeroscaler/zeroscaler/jsii"

	"github.com/aws/constructs-go/constructs/v10"
	"github.com/lephyrius/zeroscaler/zeroscaler/internal"
)

type Zeroscaler interface {
	constructs.Construct
	// The tree node.
	Node() constructs.Node
	// Returns a string representation of this construct.
	ToString() *string
}

// The jsii proxy struct for Zeroscaler
type jsiiProxy_Zeroscaler struct {
	internal.Type__constructsConstruct
}

func (j *jsiiProxy_Zeroscaler) Node() constructs.Node {
	var returns constructs.Node
	_jsii_.Get(
		j,
		"node",
		&returns,
	)
	return returns
}


func NewZeroscaler(scope constructs.Construct, id *string, props *ZeroscalerProps) Zeroscaler {
	_init_.Initialize()

	if err := validateNewZeroscalerParameters(scope, id, props); err != nil {
		panic(err)
	}
	j := jsiiProxy_Zeroscaler{}

	_jsii_.Create(
		"@zeroscaler/zeroscaler-cdk.Zeroscaler",
		[]interface{}{scope, id, props},
		&j,
	)

	return &j
}

func NewZeroscaler_Override(z Zeroscaler, scope constructs.Construct, id *string, props *ZeroscalerProps) {
	_init_.Initialize()

	_jsii_.Create(
		"@zeroscaler/zeroscaler-cdk.Zeroscaler",
		[]interface{}{scope, id, props},
		z,
	)
}

// Checks if `x` is a construct.
//
// Use this method instead of `instanceof` to properly detect `Construct`
// instances, even when the construct library is symlinked.
//
// Explanation: in JavaScript, multiple copies of the `constructs` library on
// disk are seen as independent, completely different libraries. As a
// consequence, the class `Construct` in each copy of the `constructs` library
// is seen as a different class, and an instance of one class will not test as
// `instanceof` the other class. `npm install` will not create installations
// like this, but users may manually symlink construct libraries together or
// use a monorepo tool: in those cases, multiple copies of the `constructs`
// library can be accidentally installed, and `instanceof` will behave
// unpredictably. It is safest to avoid using `instanceof`, and using
// this type-testing method instead.
//
// Returns: true if `x` is an object created from a class which extends `Construct`.
func Zeroscaler_IsConstruct(x interface{}) *bool {
	_init_.Initialize()

	if err := validateZeroscaler_IsConstructParameters(x); err != nil {
		panic(err)
	}
	var returns *bool

	_jsii_.StaticInvoke(
		"@zeroscaler/zeroscaler-cdk.Zeroscaler",
		"isConstruct",
		[]interface{}{x},
		&returns,
	)

	return returns
}

func (z *jsiiProxy_Zeroscaler) ToString() *string {
	var returns *string

	_jsii_.Invoke(
		z,
		"toString",
		nil, // no parameters
		&returns,
	)

	return returns
}

