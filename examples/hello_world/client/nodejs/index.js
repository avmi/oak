const fs = require('fs');
const protobufjs = require('protobufjs');
const grpc = require('@grpc/grpc-js');
const grpcProtoLoader = require('@grpc/proto-loader');

const CERT_PATH = __dirname + '/../../../certs/local/ca.pem';
const SERVICE_PROTO_PATH = __dirname + '/../../proto/hello_world.proto';
const OAK_LABEL_PROTO_PATH = __dirname + '/../../../../oak/proto/label.proto';

// Keep in sync with /oak/server/rust/oak_runtime/src/node/grpc/server/mod.rs.
const oakLabelGrpcMetadataKey = 'x-oak-label-bin';

async function main() {
  const [oaklabelDefinition, helloWorldDefinition] = await Promise.all([
    // We use two different libs with a similar APIs to load proto files,
    // which due to the design of the `@grpc/grpc-js` lib.
    // `@grpc/proto-loader` is a wrapper around `protobufjs` that adds
    // functionality required for working with `grpc-js`, but omits others.
    // Hence we use grpcProtoLoader for gRPC services, and protobufjs
    // for all other protos.
    protobufjs.load(OAK_LABEL_PROTO_PATH),
    grpcProtoLoader.load(SERVICE_PROTO_PATH),
  ]);

  function getGrpcMetadata() {
    // TODO(#1097): move this into an SDK package to allow re-use.
    const OakLabel = oaklabelDefinition.lookupType('oak.label.Label');

    // TODO(#1066): Use a more restrictive Label.
    const publicUntrustedLabel = OakLabel.create({});
    const encodedLabel = OakLabel.encode(publicUntrustedLabel).finish();

    const metaData = new grpc.Metadata();
    metaData.add(oakLabelGrpcMetadataKey, encodedLabel);

    return metaData;
  }

  const helloWorldProto = grpc.loadPackageDefinition(helloWorldDefinition).oak
    .examples.hello_world;
  const credentials = grpc.credentials.createSsl(fs.readFileSync(CERT_PATH));

  const client = new helloWorldProto.HelloWorld('localhost:8080', credentials);

  // For documentation on client calls see the `@grpc/grpc-js` documentation:
  // https://grpc.github.io/grpc/node/grpc.Client.html#~CallProperties
  client.sayHello(
    // The arguments passed to the gRPC service. Corresponds to the
    // `HelloRequest` message type in hello_world.proto file.
    { greeting: 'Node.js' },
    // The metadata for this gRPC call.
    getGrpcMetadata(),
    // Callback invoked with the response.
    (error, response) => {
      if (error) {
        console.error(error);
        process.exit(1);
      } else {
        console.log(response.reply);
        process.exit(0);
      }
    }
  );
}

main();
