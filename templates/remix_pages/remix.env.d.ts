/// <reference types="@remix-run/dev" />
/// <reference types="@remix-run/cloudflare" />
/// <reference types="@cloudflare/workers-types" />

import type PocketBase from "pocketbase";

declare module "@remix-run/server-runtime" {
  interface AppLoadContext {
    client: PocketBase;
    production: boolean;
  }
}
