<template>
  <main>
    <section class="primary-section">
      <div id="title">
        <div class="description-container">
          <img src="~/assets/images/logos/aragog-logo-blanc.svg" alt="aragog-logo">
          <p>{{ $t('index.blocs.description') }}</p>
          <a href="https://qonfucius.gitlab.io/aragog/" target="_blank">
            <a-button :invert-color="true" :large="true">{{ $t('index.blocs.buttons.quick-start') }}</a-button>
          </a>
          <a href="https://gitlab.com/qonfucius/aragog" target="_blank">
            <a-button :shadow="true" :large="true">Gitlab</a-button>
          </a>
        </div>
        <div class="animate-container">
          <client-only>
            <transition appear name="aragog-picture-left">
              <img src="~/assets/images/pictures/aragog-illustration-accolade-droite.svg" class="aragog-picture-left" alt="aragog-picture-left">
            </transition>
            <transition appear name="aragog-picture-right">
              <img src="~/assets/images/pictures/aragog-illustration-accolade-gauche.svg" class="aragog-picture-right" alt="aragog-picture-right">
            </transition>
            <transition appear name="aragog-picture-ecllipse">
              <img src="~/assets/images/pictures/aragog-illustration-ellipse.svg" class="aragog-picture-ecllipse" alt="aragog-picture-ecllipse">
            </transition>
            <transition appear name="aragog-picture-screen-1">
              <img src="~/assets/images/pictures/aragog-illustration-screen-1.svg" class="aragog-picture-screen-1" alt="aragog-picture-screen">
            </transition>
            <transition appear name="aragog-picture-screen-2">
              <img src="~/assets/images/pictures/aragog-illustration-screen-2.svg" class="aragog-picture-screen-2" alt="aragog-picture-screen">
            </transition>
            <transition appear name="aragog-spider">
              <img src="~/assets/images/pictures/aragog-illustration-spider.svg" class="aragog-spider" alt="aragog-spider">
            </transition>
          </client-only>
        </div>
      </div>
    </section>
    <section class="secondary-section">
      <div id="advantages">
        <div id="list">
          <div
            v-for="(card, index) in advantages"
            :key="`advantages-card${index}`"
          >
            <img :src="require(`~/assets/images/icons/${card.logoName}.svg`)" :alt="card.logoAlt">
            <div>
              <h2>{{ card.title }}</h2>
              <p>{{ card.description }}</p>
            </div>
          </div>
        </div>
        <div id="examples">
          <h2>{{ $t('index.blocs.examples') }}</h2>
          <examples-codes :list-buttons="examplesCodes.listExamples" :list-examples="examplesCodes.examples" />
        </div>
      </div>
    </section>
  </main>
</template>

<script lang="ts">
import Vue from 'vue'

import ExamplesCodes from '~/components/elements/examples-codes';
import AButton from '~/components/elements/button';

interface Advantage {
  title: string;
  description: string;
  logoName: string;
  logoAlt: string;
}

interface Example {
  subtitle: string;
  countLanguages: number;
  rust: string;
  aql?: string;
}

interface State {
  advantages: Advantage[];
  examplesCodes: {
    listExamples: string[],
    examples: Example[]
  }
}

export default Vue.extend({
  name: 'CommunityPage',
  components: {
    ExamplesCodes,
    AButton,
  },
  data(): State {
    return {
      advantages: [
        {
          title: this.$t('index.blocs.safe').toString(),
          description: this.$t('index.blocs.safe-explanation').toString(),
          logoName: "safe",
          logoAlt: "safe",
        },
        {
          title: this.$t('index.blocs.easy').toString(),
          description: this.$t('index.blocs.easy-explanation').toString(),
          logoName: "easy",
          logoAlt: "easy",
        },
        {
          title: this.$t('index.blocs.productive').toString(),
          description: this.$t('index.blocs.productive-explanation').toString(),
          logoName: "productive",
          logoAlt: "productive",
        }
      ],
      examplesCodes: {
        listExamples: [
          this.$t('index.blocs.models').toString(),
          this.$t('index.blocs.hooks').toString(),
          this.$t('index.blocs.validations').toString(),
          this.$t('index.blocs.type-safe-queries').toString(),
          this.$t('index.blocs.graph-queries').toString(),
        ],
        examples: [
          {
            subtitle: this.$t('index.blocs.examples-list.0').toString(),
            rust: "```rust\n" +
              "\n#[derive(Record, Clone, Serialize, Deserialize)] \n" +
              "pub struct User { \n" +
              "    username: String, \n" +
              "    name: String, \n" +
              "    age: u16\n" +
              "}\n" +
              "```",
            countLanguages: 1,
          },
          {
            subtitle: this.$t('index.blocs.examples-list.1').toString(),
            rust: "```rust\n" +
              "\n#[derive(Record, Clone, Serialize, Deserialize)] \n" +
              "#[hook(before_save(func = « my_method »))] \n" +
              "#[hook(after_delete(func = « my_other_method »))] \n" +
              "pub struct User {\n" +
              "    username: String, \n" +
              "    name: String, \n" +
              "    age: u16\n" +
              "}\n" +
              "```",
            countLanguages: 1,
          },
          {
            subtitle: this.$t('index.blocs.examples-list.2').toString(),
            rust: "```rust\n" +
              "\n#[derive(Record, Clone, Serialize, Deserialize)] \n" +
              "#[hook(before_write(func = « validate »))] \n" +
              "pub struct User { \n" +
              "    #[validate(min_length = 5, max_length = 15)] \n" +
              "    username: String, \n" +
              "    #[validate(max_length = 50)] \n" +
              "    name: String, \n" +
              "    #[validate(greater_than(18))] \n" +
              "    age: u16\n" +
              "}\n" +
              "```",
            countLanguages: 1,
          },
          {
            subtitle: this.$t('index.blocs.examples-list.3').toString(),
            rust: "```rust\n" +
              "\nQuery::new(“Companies“) \n" +
              "    .filter(Filter::new(Comparison::any(“emails“).like(“%gmail.com“)))\n" +
              "    .sort(“company_name“, None)\n" +
              "    .sort(“company_age“, Some(SortDirection::Desc))\n" +
              "    .limit(5, None)\n" +
              "    .distinct();\n" +
              "```",
            aql: "```aql\n " +
              "\nFOR a in Companies \n " +
              "    FILTER a.emails ANY LIKE “%gmail.com“\n " +
              "    SORT a.company_name ASC, a.company_age DESC\n " +
              "    LIMIT 5\n" +
              "    return DISTINCT a\n " +
              "```",
            countLanguages: 2,
          },
          {
            subtitle: this.$t('index.blocs.examples-list.4').toString(),
            rust: "```rust\n" +
              "\nQuery::new(“Companies“)\n " +
              "    .filter(Filter::new(Comparison::any(“emails“).like(“%gmail.com“)))\n " +
              "    .sort(“company_name“, None)\n " +
              "    .join_outbound(1, 2, false,\n " +
              "        Query::new(“MemberOf“)\n " +
              "            .sort(“_id“, None)\n " +
              "            .prune(\n " +
              "                Comparison::statement(“1“).equals(1).into()\n " +
              "            ), \n " +
              "    );\n" +
              "```",
            aql: "```aql\n " +
              "\nFOR a in Companies \n " +
              "    FILTER a.emails ANY LIKE “%gmail.com“ \n " +
              "    SORT b.company_name ASC \n " +
              "        FOR a in 1..2 OUTBOUND b MemberOf \n" +
              "            SORT a._id ASC \n" +
              "            PRUNE 1 == 1 \n" +
              "            return a\n" +
              "```",
            countLanguages: 2,
          },
        ],
      }
    };
  },
  head(this: any) {
    return {
      title: this.$t('index.title'),
      meta: [{ hid: 'index.description', name: 'description', content: this.$t('index.description') }],
    };
  },
})
</script>

<style scoped lang="scss">
@import 'assets/css/_vars.scss';

.aragog-picture-left-enter-active,
.aragog-picture-right-enter-active,
.aragog-picture-ecllipse-enter-active,
.aragog-picture-screen-1-enter-active,
.aragog-picture-screen-2-enter-active,
.aragog-spider-enter-active {
  transition: all 2s;
}
.aragog-picture-left-enter {
  transform: translate(50px, 30px);
}

.aragog-picture-right-enter,
.aragog-picture-screen-1-enter  {
  transform: translate(-50px, 50px);
}

.aragog-picture-ecllipse-enter {
  transform: translate(-50px, -20px);
}

.aragog-picture-screen-2-enter {
  transform: translate(40px, -20px);
}

.aragog-spider-enter {
  transform: translateY(50px);
}

main {
  section {
    #title {
      min-height: 500px;
      height: 60vh;
      .description-container {
        color: $white;
        text-align: center;
        padding: $size-large $size-extra-small;

        position: absolute;
        top: 50%;
        left: 0;
        right: 0;
        bottom: 0;
        transform: translateY(-50%);

        img {
          width: 250px;
        }

        p {
          font-size: $size-medium;
        }

        a {
          .button {
            margin-right: $size-small;
          }
        }
      }
      .animate-container {
        display: none;
      }
    }
    #advantages {
      min-height: 60vh;
      margin-bottom: $size-extra-large * 4;
      #list {
        padding: $size-extra-large 0;
        div {
          img {
            height: 100px;
            width: 100px;
            padding-top: $size-small;
            float: left;
          }
          div {
            overflow: auto;
            min-width: 150px;
          }
        }
      }
    }
  }
}

@media screen and(min-width: $tablet) {
  main {
    section {
      #title {
        display: grid;
        grid-auto-rows: auto;
        grid-gap: $size-extra-large;
        grid-template-columns: 1.5fr 1.5fr;
        align-content: center;
        overflow: hidden;
        min-height: 700px;
        max-height: 1000px;
        .description-container {
          position: relative;
          text-align: left;
          margin-left: 10%;
          img {
            width: 350px;
          }
        }
        .animate-container {
          position: relative;
          display: block;
          width: 500px;
          margin-top: -180px;
          img {
            width: 200px;
            position: absolute;
          }

          .aragog-picture-left {
            z-index: 3;
            right: -120px;
            top: 120px;
            width: 60px;
          }
          .aragog-picture-right {
            z-index: 5;
            width: 60px;
            top: 330px;
            left: 80px;
          }
          .aragog-picture-ecllipse {
            z-index: 1;
            left: 100px;
            width: 200px;
          }
          .aragog-picture-screen-1 {
            z-index: 4;
            top: 160px;
            left: -30px;
            width: 420px;
          }
          .aragog-picture-screen-2 {
            z-index: 2;
            width: 600px;
            height: 600px;
          }
          .aragog-spider {
            z-index: 6;
            width: 156px;
            right: -50px;
            top: -120px;
          }
        }
      }
      #advantages {
        margin-top: $size-extra-large * 2;
        #list {
          display: grid;
          grid-auto-rows: auto;
          grid-gap: $size-extra-large;
          grid-template-columns: repeat(3, 1fr);
        }
        #examples {
          margin: $size-extra-large * 3 0;
        }
      }
    }
  }
}

@media screen and(max-width: 1500px) and (min-width: $tablet) {
  main {
    .secondary-section {
      padding: $size-large * 3;
    }
  }
}
</style>

<i18n src="./index.i18n.yml" lang="yaml" />
