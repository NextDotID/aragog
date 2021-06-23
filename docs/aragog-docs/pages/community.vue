<template>
  <main>
    <section class="secondary-section">
      <div id="community">
        <h1 class="title">{{ $t('community.blocs.title') }}</h1>
        <div
          v-for="(card, index) in community"
          :key="`community-card${index}`"
        >
          <h2>{{ card.title }}</h2>
          <p>{{ card.description }}</p>
          <a
            class="link"
            :href="card.link"
            target="_blank"
          >
            {{ card.link }}
          </a>
        </div>

        <div class="contact">
          <h2>{{ $t('community.blocs.contact') }}</h2>
          <div>
            <p class="developer">{{ $t('community.blocs.developer') }}</p>
            <p>Félix de Maneville</p>
            <p>felix.maneville@qonfucius.team</p>
          </div>
          <div>
            <p class="team">{{ $t('community.blocs.team-qonfucius') }}</p>
            <p>contact@qonfucius.team</p>
          </div>
        </div>
      </div>
    </section>
  </main>
</template>

<script lang="ts">
import Vue from 'vue';

interface community {
  title: string;
  description: string;
  link: string;
}

interface State {
  community: community[];
}

export default Vue.extend({
  name: 'CommunityPage',
  data(): State {
    return {
      community: [
        {
          title: this.$t('community.blocs.code-repository').toString(),
          description: this.$t('community.blocs.code-repository-explanation').toString(),
          link: 'https://gitlab.com/qonfucius/aragog',
        },
        {
          title: this.$t('community.blocs.chat-room').toString(),
          description: this.$t('community.blocs.chat-room-explanation').toString(),
          link: 'https://gitter.im/aragog-rs/community',
        },
        {
          title: this.$t('community.blocs.blog').toString(),
          description: this.$t('community.blocs.blog-explanation').toString(),
          link: 'http://blog.qongzi.com/',
        },
      ],
    }
  },
  head(this: any) {
    return {
      title: this.$t('community.title'),
      meta: [{ hid: 'community.description', name: 'description', content: this.$t('community.description') }],
    };
  },
});
</script>

<style scoped lang="scss">
@import 'assets/css/vars';

main {
  > section {
    min-height: 90vh;
  }
  #community {
    div {
      margin-bottom: $size-extra-large * 2;
    }
    .contact {
      .developer,
      .team {
        font-weight: bold;
      }
      div {
        margin-bottom: $size-extra-large;
      }
      p {
        line-height: 0.2;
      }
    }

  }
}
</style>

<i18n src="./community.i18n.yml" lang="yaml" />
