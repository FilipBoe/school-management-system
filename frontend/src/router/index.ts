import { watchOnce } from "@vueuse/core";
import { RedirectToSignIn, useAuth, useClerk, useSignIn } from "vue-clerk";
import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            component: () => import("../components/layouts/Layouts.vue"),
            children: [
                {
                    path: "/",
                    name: "home",
                    component: () => import("../views/HomeView.vue"),
                    meta: {
                        auth: true,
                    },
                },
                {
                    path: "/users",
                    name: "Users",
                    component: () => import("../views/AboutView.vue"),
                },
                {
                    path: "/:pathMatch(.*)*",
                    name: "NotFound",
                    component: () => import("../views/errors/NotFound.vue"),
                },
            ],
        },
    ],
});

export default router;
