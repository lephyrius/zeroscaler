# Zeroscaler CDK Construct

This is a CDK Construct Library that scales up Fargate containers based when they are at zero.

The construct defines an interface (`ZeroscalerProps`) to configure the Zeroscaler.

## Example

```typescript
import { Zeroscaler } from './lib';

new Zeroscaler(stack, 'MyZeroscaler', {
  targetGroupArn: 'arn:aws:elasticloadbalancing:...',
  fargateTaskArn: 'arn:aws:ecs:...',
  // Optionally override vpc or cluster
});
```


## Useful commands

* `npm run build`   compile typescript to js
* `npm run watch`   watch for changes and compile
* `npm run test`    perform the jest unit tests
