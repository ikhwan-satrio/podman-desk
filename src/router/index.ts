import { createRouter, createWebHistory } from "vue-router";
import { routes, handleHotUpdate } from "vue-router/auto-routes";

export const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

// update routes at runtime without reloading the page
if (import.meta.hot) {
  handleHotUpdate(router);
}
