import type { ActionFunction, LoaderFunction } from "@remix-run/cloudflare";
import { redirect } from "@remix-run/cloudflare";

export function adminsOnly(
  func: LoaderFunction | ActionFunction,
  path?: string
): LoaderFunction | ActionFunction {
  return async function ({ request, context, params }) {
    if (context.client.authStore.token) {
      // if (context.client.authStore.model?.collectionName != "admins") {
      //   return redirect("/admin/login");
      // }
      return await func({ request, context, params });
    } else {
      return redirect(path || "/admin/login", { status: 302 });
    }
  };
}

export function loggedInOnly(
  func: LoaderFunction | ActionFunction,
  path?: string
): LoaderFunction | ActionFunction {
  return async function ({ request, context, params }) {
    if (context.client.authStore.token) {
      return await func({ request, context, params });
    } else {
      return redirect(path || "/login", { status: 302 });
    }
  };
}
