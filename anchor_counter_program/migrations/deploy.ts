//THIS IS NOT NEEDED FOR COUNTER PROGRAM, THIS IS JUST A TEMPLATE FOR DEPLOYMENT SCRIPT GEENRATED BY ANCHOR CLI.
// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

import * as anchor from "@coral-xyz/anchor";

module.exports = async function (provider: anchor.AnchorProvider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here.
};
