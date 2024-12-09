<script setup lang="ts">
import { useCreateApiFetch } from "@/composables/useApiFetch";
import { config } from "@/config";
import { useAuthTokenStore } from "@/stores/authTokenStore";
import type { User } from "@/types";
import Button from "primevue/button";
import { onMounted, ref } from "vue";
import { useClerk } from "vue-clerk";

const apiFetch = useCreateApiFetch();
const authTokenStore = useAuthTokenStore();
const { redirectToSignIn } = useClerk();
const users = ref<User[]>([]);
let retried = false;

const url = config.apiUrl + "api/v1/users";
const {
    isFetching,
    error,
    statusCode,
    onFetchResponse,
    onFetchError,
    execute,
} = apiFetch(url);

onFetchResponse(async (response) => {
    users.value = await response.json();
});

onFetchError(async () => {
    if (statusCode.value === 401) {
        if (!retried) {
            await new Promise((resolve) => setTimeout(resolve, 1000));
            await authTokenStore.updateToken();
            execute();
        } else {
            redirectToSignIn({
                redirectUrl: window.location.href,
            });
        }

        retried = true;
    }
});

const retry = () => {
    retried = false;
    execute();
};
</script>

<template>
    <main>
        <h1 class="text-3xl font-bold">Home</h1>

        <Button severity="primary" label="Fetch Users" @click="retry"></Button>

        <div class="my-5">
            IsFetching: {{ isFetching }} <br />
            error: {{ error }} <br />
        </div>

        <div v-for="user in users">{{ user.id }}: {{ user.name }}</div>
    </main>
</template>
