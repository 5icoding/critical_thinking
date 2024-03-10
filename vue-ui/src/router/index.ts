import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";

const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        name: 'Login',
        // 异步加载，打包时代码分割，性能优化
        component: () => import('../views/login.vue') 
    },
    {
        path: '/reg',
        name: 'Reg',
        component: () => import('../views/reg.vue')
    },
    {
        path: '/pdf',
        name: 'Pdf',
        component: () => import('../views/pdf.vue')
    },
    {
        path: '/pdf2',
        name: 'Pdf2',
        component: () => import('../views/pdf2.vue')
    }
]

const router = createRouter({
    history: createWebHistory(), // 路由模式：history模式
    routes: routes
})

export default router;