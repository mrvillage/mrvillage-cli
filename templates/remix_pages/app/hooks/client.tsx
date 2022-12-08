import { Record } from "pocketbase";
import { Admin } from "pocketbase";
import PocketBase, { BaseAuthStore } from "pocketbase";
import { createContext, useContext, useEffect, useMemo, useState } from "react";

// TAKEN FROM Pocketbase's source code,
// https://github.com/pocketbase/js-sdk/blob/7956bfe992cea931a120f039e0d50c479e910e84/src/stores/utils/cookie.ts#L25
export interface ParseOptions {
  decode?: (val: string) => string;
}

function defaultDecode(val: string): string {
  return val.indexOf("%") !== -1 ? decodeURIComponent(val) : val;
}

export function cookieParse(
  str: string,
  options?: ParseOptions
): { [key: string]: any } {
  const result: { [key: string]: any } = {};

  if (typeof str !== "string") {
    return result;
  }

  const opt = Object.assign({}, options || {});
  const decode = opt.decode || defaultDecode;

  let index = 0;
  while (index < str.length) {
    const eqIdx = str.indexOf("=", index);

    // no more cookie pairs
    if (eqIdx === -1) {
      break;
    }

    let endIdx = str.indexOf(";", index);

    if (endIdx === -1) {
      endIdx = str.length;
    } else if (endIdx < eqIdx) {
      // backtrack on prior semicolon
      index = str.lastIndexOf(";", eqIdx - 1) + 1;
      continue;
    }

    const key = str.slice(index, eqIdx).trim();

    // only assign once
    if (undefined === result[key]) {
      let val = str.slice(eqIdx + 1, endIdx).trim();

      // quoted values
      if (val.charCodeAt(0) === 0x22) {
        val = val.slice(1, -1);
      }

      try {
        result[key] = decode(val);
      } catch (_) {
        result[key] = val; // no decoding
      }
    }

    index = endIdx + 1;
  }

  return result;
}

// BACK TO CODE I WROTE, BASED LARGELY ON Pocketbase's IMPLEMENTATION OF LOCAL AUTH STORE
// https://github.com/pocketbase/js-sdk/blob/rc/src/stores/LocalAuthStore.ts
class AuthStore extends BaseAuthStore {
  private fallback: { [key: string]: any } = {};
  private key: string;

  constructor(key?: string) {
    super();

    this.key = key || "pb_auth";
  }

  /**
   * @inheritdoc
   */
  get token(): string {
    return (this._get(this.key) || {}).token || "";
  }

  /**
   * @inheritdoc
   */
  get model(): Record | Admin | null {
    const data = this._get(this.key) || {};
    if (
      data === null ||
      typeof data !== "object" ||
      data.model === null ||
      typeof data.model !== "object"
    ) {
      return null;
    }
    if (typeof data.model?.collectionId === "undefined") {
      return new Admin(data.model);
    }
    return new Record(data.model);
  }

  /**
   * @inheritdoc
   */
  save(token: string, model: Record | Admin | null): void {
    this._set(this.key, { token, model });
    super.save(token, model);
  }

  /**
   * @inheritdoc
   */
  clear(): void {
    this._remove(this.key);
    super.clear();
  }

  private _get(key: string) {
    if (typeof document !== "undefined") {
      const data = cookieParse(document.cookie || "")[key] || "";
      if (data) {
        return JSON.parse(data);
      }
    }
    return this.fallback[key];
  }

  private _set(key: string, value: any) {
    if (typeof document !== "undefined") {
      document.cookie = this.exportToCookie();
    }
    this.fallback[key] = value;
  }

  private _remove(key: string) {
    if (typeof document !== "undefined") {
      document.cookie = this.exportToCookie(
        {
          expires: new Date(0),
        },
        key
      );
    }
    delete this.fallback[key];
  }

  exportToCookie(options?: any, key?: string | undefined): string {
    return super.exportToCookie(
      {
        ...options,
        httpOnly: false,
        secure:
          typeof window === "undefined"
            ? process.env.NODE_ENV === "production"
            : window.location.hostname !== "localhost" &&
              window.location.hostname !== "127.0.0.1",
      },
      key || this.key
    );
  }
}

function createClient(loaderCookie: string) {
  const client = new PocketBase(
    (
      typeof window === "undefined"
        ? process.env.NODE_ENV === "production"
        : window.location.hostname !== "localhost" &&
          window.location.hostname !== "127.0.0.1"
    )
      ? "$$-cli-api-domain-$$"
      : "http://localhost:8090",
    new AuthStore()
  );
  client.authStore.loadFromCookie(
    typeof document === "undefined" ? loaderCookie : document.cookie
  );
  return client;
}

const ClientContext = createContext<PocketBase>(null as any);

export const ClientProvider = ({
  children,
  loaderCookie,
}: {
  children: React.ReactNode;
  loaderCookie: string;
}) => {
  const client = useMemo(() => createClient(loaderCookie), [loaderCookie]);
  return (
    <ClientContext.Provider value={client}>{children}</ClientContext.Provider>
  );
};

export function useClient() {
  return useContext(ClientContext);
}

export function useLoggedIn() {
  const client = useClient();

  const [loggedIn, setLoggedIn] = useState(!!client.authStore.token);

  useEffect(() => {
    client.authStore.onChange(() => setLoggedIn(!!client.authStore.token));
    setLoggedIn(!!client.authStore.token);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return loggedIn;
}

export function useIsAdmin() {
  const client = useClient();

  const [isAdmin, setIsAdmin] = useState(
    client.authStore.model?.collectionName === "admins"
  );

  useEffect(() => {
    client.authStore.onChange(() =>
      setIsAdmin(client.authStore.model instanceof Admin)
    );
    setIsAdmin(client.authStore.model instanceof Admin);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
  return isAdmin;
}
