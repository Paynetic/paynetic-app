import { createRouter, createWebHistory } from 'vue-router'

declare module 'vue-router' {
  interface RouteMeta {
    title?: string
    noScroll?: boolean
    scrollAnchor?: string
    requiresAuth?: boolean
  }
}

const metaAuth = (title: string) => ({
  title,
  requiresAuth: true,
})

const router = createRouter({
  history: createWebHistory(),
  scrollBehavior(to, from, savedPosition) {
    if (to?.hash) {
      return { el: to.hash }
    }
    if (savedPosition) {
      return savedPosition
    }
    if (to?.meta?.noScroll && from?.meta?.noScroll) {
      return {}
    }
    return { top: 0 }
  },
  routes: [
    {
      path: '/',
      component: () => import('./views/HomeWrap.vue'),
      children: [
        {
          path: '',
          name: 'Home',
          component: () => import('./views/Home.vue'),
        },
        {
          path: '/about',
          name: 'About',
          component: () => import('./views/About.vue'),
        },
        {
          path: '/faq',
          name: 'Faq',
          component: () => import('./views/Faq.vue'),
        },
        {
          path: '/integration',
          name: 'Integration',
          component: () => import('./views/Integration.vue'),
        },
        {
          path: '/features',
          name: 'Features',
          component: () => import('./views/Features.vue'),
        },
        {
          path: '/connect',
          name: 'Connect',
          component: () => import('./views/Connect.vue'),
        },
        {
          path: '/terms',
          name: 'Terms',
          component: () => import('./views/Terms.vue'),
        },
        {
          path: '/cookies',
          name: 'Cookies',
          component: () => import('./views/Cookies.vue'),
        },
        {
          path: '/privacy',
          name: 'Privacy',
          component: () => import('./views/Privacy.vue'),
        },
        {
          path: '/trust',
          name: 'Trust',
          component: () => import('./views/Trust.vue'),
        },
      ],
    },
    {
      path: '/profile',
      component: () => import('./views/AppWrap.vue'),
      children: [
        {
          path: '',
          name: 'Profile',
          component: () => import('./views/Profile.vue'),
          redirect: '/profile/subscriptions',
          meta: metaAuth('Profile'),
          children: [
            {
              path: 'subscriptions',
              name: 'Subscriptions',
              component: () => import('./components/profile/Subscriptions.vue'),
            },
            {
              path: 'products',
              name: 'Products',
              component: () => import('./components/profile/Products.vue'),
            },
            {
              path: 'settings',
              name: 'Settings',
              component: () => import('./components/profile/Settings.vue'),
            },
          ],
        },
        {
          path: '/browse',
          name: 'Browse',
          component: () => import('./views/Products.vue'),
        },
        {
          path: '/product/:id',
          name: 'Product',
          component: () => import('./views/Product.vue'),
        },
        {
          path: '/create',
          name: 'Create',
          component: () => import('./views/Create.vue'),
        },
      ],
    },
  ],
})

router.afterEach((to, _from) => {
  const parent = to.matched.find((record) => record.meta.title)
  const parentTitle = parent ? parent.meta.title : null
  document.title = to.meta.title || parentTitle || 'Paynetic'
})

export default router
