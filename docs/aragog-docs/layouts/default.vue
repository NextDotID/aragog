<template>
  <div>
    <MenuDesktop :is-active="isMenuActive" @toggle="toggleMenu" />
    <MenuMobile :is-active="isMenuActive" />
    <Nuxt />
    <Footer />
  </div>
</template>

<script lang="ts">
import Vue from 'vue';

import MenuDesktop from '~/components/modules/menu/menu-desktop.vue';
import MenuMobile from '~/components/modules/menu/menu-mobile.vue';
import Footer from '~/components/modules/footer';


export default Vue.extend({
  components: {
    MenuDesktop,
    MenuMobile,
    Footer
  },
  data: () => ({
    isMenuActive: false,
  }),
  watch: {
    $route() {
      if (this.isMenuActive) {
        this.toggleMenu();
      }
    },
  },
  methods: {
    toggleMenu(): void {
      this.isMenuActive = !this.isMenuActive;
    },
  },
});
</script>
