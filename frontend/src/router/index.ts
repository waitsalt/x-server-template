import HomeView from '@/view/HomeView.vue'
import LoginView from '@/view/user/LoginView.vue'
import RegisterView from '@/view/user/RegisterView.vue'
import SettingView from '@/view/user/SettingView.vue'
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: '网站首页',
      component: HomeView,
    },
    {
      path: '/user/login',
      name: '用户登陆',
      component: LoginView,
    },
    {
      path: '/user/register',
      name: '用户注册',
      component: RegisterView,
    },
    {
      path: '/user/setting',
      name: '用户设置',
      component: SettingView,
    },
  ],
})

export default router
