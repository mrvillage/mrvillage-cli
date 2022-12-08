import React from "react";
import type { ColorScheme } from "@mantine/core";
import { ColorSchemeProvider, MantineProvider } from "@mantine/core";
import { NotificationsProvider } from "@mantine/notifications";
import { useLocalStorage } from "@mantine/hooks";
import { ModalsProvider } from "@mantine/modals";
import { SpotlightProvider } from "@mantine/spotlight";
import { IconSearch } from "@tabler/icons";

export default function Providers({ children }: { children: React.ReactNode }) {
  const [colorScheme, setColorScheme] = useLocalStorage<ColorScheme>({
    key: "colorScheme",
    defaultValue: "dark",
    getInitialValueInEffect: true,
  });

  const toggleColorScheme = (value?: ColorScheme) => {
    setColorScheme(value || (colorScheme === "dark" ? "light" : "dark"));
  };
  return (
    <ColorSchemeProvider
      colorScheme={colorScheme}
      toggleColorScheme={toggleColorScheme}
    >
      <MantineProvider
        theme={{
          colorScheme,
          spacing: { xxs: 5, xxxs: 2 },
          primaryColor: "$$-cli-primary-color-$$",
        }}
        withNormalizeCSS
        withGlobalStyles
      >
        <NotificationsProvider>
          <ModalsProvider>
            <SpotlightProvider
              actions={[]}
              searchIcon={<IconSearch size={18} />}
              searchPlaceholder="Search..."
              shortcut={["mod + P", "mod + K", "/"]}
              nothingFoundMessage="Nothing found..."
            >
              {children}
            </SpotlightProvider>
          </ModalsProvider>
        </NotificationsProvider>
      </MantineProvider>
    </ColorSchemeProvider>
  );
}
