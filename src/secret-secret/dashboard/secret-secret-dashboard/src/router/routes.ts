import Home from "@/views/Home.vue";
import CookiePolicy from "@/views/CookiePolicy.vue";
import About from "@/views/About.vue";
import Vue from "vue";
import VueRouter, { RouteConfig } from "vue-router";

// 2. Define some routes
// Each route should map to a component.
// We'll talk about nested routes later.
const routes: RouteConfig[] = [
  {
    path: "*",
    redirect: "/",
  },
  { path: "/", component: Home },
  { path: "/cookie-policy", component: CookiePolicy },
  { path: "/about", component: About },
];

// 3. Create the router instance and pass the `routes` option
// You can pass in additional options here, but let's
// keep it simple for now.
const router = new VueRouter({
  // 4. Provide the history implementation to use. We are using the hash history for simplicity here.
  routes, // short for `routes: routes`
});

Vue.use(VueRouter);
// 5. Create a
export default router;
