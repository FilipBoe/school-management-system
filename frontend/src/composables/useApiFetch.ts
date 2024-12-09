import { useAuthTokenStore } from "@/stores/authTokenStore";
import { createFetch } from "@vueuse/core";
import { storeToRefs } from "pinia";

type RetryConfig = {
    retry?: boolean;
};

export function useCreateApiFetch(
    authToken: string | null = null,
    retryConfig: RetryConfig = {}
) {
    const authTokenStore = useAuthTokenStore();
    const { token } = storeToRefs(authTokenStore);

    return createFetch({
        options: {
            async beforeFetch({ options }) {
                options.headers = {
                    ...options.headers,
                    Authorization: `Bearer ${authToken ?? token.value}`,
                };

                return {
                    options,
                };
            },
        },
    });
}
