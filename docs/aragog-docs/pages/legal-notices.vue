<template>
  <main>
    <section class="secondary-section">
      <div id="legal-notices">
        <h1 class="title">{{ $t('legal-notices.blocs.title') }}</h1>
        <p class="subtitle">{{ $t('legal-notices.blocs.subtitle') }} :</p>
        <div class="qonfucius">
          <p
            v-for="(card, index) in $t('legal-notices.blocs.qonfucius')"
            :key="`community-card${index}`"
          >
            {{ card }}
          </p>
        </div>
        <p>{{ $t('legal-notices.blocs.director-publication') }} : Mikaël DEMETTE</p>
      </div>
    </section>
  </main>
</template>

<script lang="ts">
import Vue from 'vue'

export default Vue.extend({
  name: 'LegalNoticesPage',
  head(this: any) {
    return {
      title: this.$t('legal-notices.title'),
      meta: [{ hid: 'legal-notices.description', name: 'description', content: this.$t('legal-notices.description') }],
    };
  },
})
</script>

<style scoped lang="scss">
@import 'assets/css/vars';

main {
  > section {
    min-height: 90vh;
  }
  #legal-notices {
    .qonfucius {
      padding: $size-extra-small 0;
      p {
        line-height: 1;
      }
    }
  }
}
</style>

<i18n src="./legal-notices.i18n.yml" lang="yaml" />
