import { ref } from "vue";
import { defineStore } from "pinia";
import { useAuth } from "vue-clerk";

export const useAuthTokenStore = defineStore(
    "authToken",
    () => {
        const { getToken } = useAuth();

        const token = ref<string | null>(null);

        async function updateToken() {
            token.value = await getToken.value({
                template: "backend",
            });
        }

        function clearToken() {
            token.value = null;
        }

        return {
            token,
            updateToken,
            clearToken,
        };
    },
    {
        persist: true,
    }
);
