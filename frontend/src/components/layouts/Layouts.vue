<template>
    <div>
        <Menubar :model="items" class="m-2">
            <template #start></template>
            <template #item="{ item, props, hasSubmenu, root }">
                <RouterLink
                    v-ripple
                    class="flex items-center p-2 px-3 gap-2"
                    :to="item.url ?? '#'"
                >
                    <span>{{ item.label }}</span>
                    <Badge
                        v-if="item.badge"
                        :class="{ 'ml-auto': !root, 'ml-2': root }"
                        :value="item.badge"
                    />
                    <span
                        v-if="item.shortcut"
                        class="ml-auto border border-surface rounded bg-emphasis text-muted-color text-xs p-1"
                        >{{ item.shortcut }}</span
                    >
                    <i
                        v-if="hasSubmenu"
                        :class="[
                            'pi pi-angle-down ml-auto',
                            {
                                'pi-angle-down': root,
                                'pi-angle-right': !root,
                            },
                        ]"
                    ></i>
                </RouterLink>
            </template>
            <template #end>
                <div class="flex items-center gap-2">
                    <UserButton />

                    <SignedOut>
                        <SignInButton />
                    </SignedOut>
                </div>
            </template>
        </Menubar>

        <div class="m-5">
            <router-view />
        </div>
    </div>
</template>

<script setup lang="ts">
import { useAuthTokenStore } from "@/stores/authTokenStore";
import { storeToRefs } from "pinia";
import { Badge } from "primevue";
import Menubar from "primevue/menubar";
import { watch } from "vue";
import { SignedOut, SignInButton, UserButton, useAuth } from "vue-clerk";

const items = [
    {
        label: "Home",
        url: "/",
    },
    {
        label: "Users",
        hasSubmenu: true,
        items: [
            {
                label: "Active Users",
                url: "/users/active",
            },
            {
                label: "Inactive Users",
                url: "/users/inactive",
            },
        ],
    },
];

const authTokenStore = useAuthTokenStore();
const { token: authToken } = storeToRefs(authTokenStore);
const { isLoaded, isSignedIn } = useAuth();

watch(isLoaded, async () => {
    if (!isSignedIn.value) {
        authTokenStore.clearToken();

        return;
    }

    if (!authToken.value) {
        await authTokenStore.updateToken();
        window.location.reload();
    }
});
</script>
