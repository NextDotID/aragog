import Sass from 'sass';
import i18n from './config/i18n/config';

export default {
  target: 'static',
  head: {
    titleTemplate: '%s - Aragog',
    meta: [
      { charset: 'utf-8' },
      { name: 'viewport', content: 'width=device-width, initial-scale=1' },
    ],
    htmlAttrs: {
      lang: 'en'
    },
    link: [
      { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }
    ]
  },

  router: {
    trailingSlash: true,
  },

  css: [{ src: '~assets/css/main.scss', lang: 'scss' }],

  components: true,

  buildModules: [
    '@nuxt/typescript-build',
    '@nuxtjs/svg'
  ],

  modules: [
    ['nuxt-i18n', i18n],
    '@nuxtjs/sitemap',
  ],

  build: {
    extractCSS: true,
    loaders: {
      scss: {
        implementation: Sass,
      },
    },
  },
  sitemap: {
    hostname: process.env.CUSTOM_DOMAIN,
  }
}
