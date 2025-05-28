//go:build no_runtime_type_checking

package zeroscaler

// Building without runtime type checking enabled, so all the below just return nil

func validateZeroscaler_IsConstructParameters(x interface{}) error {
	return nil
}

func validateNewZeroscalerParameters(scope constructs.Construct, id *string, props *ZeroscalerProps) error {
	return nil
}

