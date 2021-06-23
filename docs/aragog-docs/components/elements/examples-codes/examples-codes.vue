<template>
  <div class="examples-codes">
    <div class="listButtons">
      <ul>
        <li
          v-for="(listItems, index) in listButtons"
          :key="`buttons-card${index}`"
          :class="{ active: isButtonActive(index) }"
          @click="changeActiveButton(index)"
        >
          {{ listItems }}
        </li>
      </ul>
    </div>
    <div class="codes">
      <div
        v-for="(example, index) in examples"
        :key="`example-card${index}`"
      >
        <template v-if="isButtonActive(index)">
          <p>
            {{ example.subtitle }}
          </p>

          <div v-if="example.countLanguages === 1">
            <div class="hljs code" ref="hlDiv" v-html="example.rust" />
          </div>
          <div v-else>
            <p class="code-title">Rust</p>
            <div class="hljs code" v-html="example.rust" />

            <p class="code-title">AQL</p>
            <div class="hljs code" v-html="example.aql" />
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue';
import marked from 'marked';
import highlight from 'highlight.js';
import 'highlight.js/styles/base16/railscasts.css';

interface Example {
  subtitle: string;
  countLanguages: number;
  rust: string;
  aql?: string;
}

interface State {
  activeButton: number;
  examples: Example[];
}

export default Vue.extend({
  name: 'Header',
  props: {
    listButtons: {
      type: Array,
      required: true,
    },
    listExamples: {
      type: Array,
      required: true,
    }
  },
  data(): State {
    return {
      activeButton: 0,
      examples: [],
    }
  },
  methods: {
    changeActiveButton(index: number): void {
      this.activeButton = index;
    },
    isButtonActive(index: number): Boolean {
      return this.activeButton === index;
    },
    replaceCountLine(line: string): string {
      let replaceLine = line;
      const count = (line.match(/\n/g) || []).length;
      let i = 0;
      replaceLine = replaceLine.replace(/\n/g, function (match) {
        i++;
        return i < count - 1 ? '\n' + i + '  ': match;
      });
      return replaceLine;
    },
  },
  mounted(){
    marked.setOptions({
        renderer: new marked.Renderer(),
        highlight: function(code) {
          return highlight.highlightAuto(code).value;
        },
        pedantic: false,
        gfm: true,
        breaks: false,
        sanitize: false,
        smartLists: true,
        smartypants: false,
        xhtml: false
      }
    );
    const listExamples = this.listExamples as Example[];
    for (const example of listExamples) {
      if(example.countLanguages === 1) {
        this.examples.push({
          subtitle: example.subtitle,
          countLanguages: example.countLanguages,
          rust: this.replaceCountLine(marked(example.rust)),
        });
      } else {
        this.examples.push({
          subtitle: example.subtitle,
          countLanguages: example.countLanguages,
          rust: this.replaceCountLine(marked(example.rust)),
          aql: this.replaceCountLine(marked(example.aql!)),
        });
      }
    }
  },
});
</script>

<style scoped lang="scss">
@import 'assets/css/_vars.scss';

.code {
  overflow: scroll;
  padding: $size-extra-small $size-large $size-normal;
}

.code-title {
  font-weight: bold;
}

.examples-codes {
  .listButtons {
    ul {
      list-style-type: none;
      padding: 0;
      margin: 0;
      li {
        background-color: #f9f9fe;
        border: 0.5px solid #F2F2FF;
        padding: $size-small 0 $size-small $size-large;
        padding-right: $size-large;
        cursor: pointer;

        &.active {
          color: $purple;
          background-color: #f2f2fc;
          border-left-color: $purple;
          border-left-width: 4px;
        }
      }
    }
  }
  .codes {
    padding: 0 $size-extra-large $size-extra-large;
    background-color: #f2f2fc;
  }
}

@media screen and(min-width: $tablet) {
  .examples-codes {
    display: grid;
    grid-auto-rows: auto;
    grid-template-columns: 0.5fr 2fr;
  }
}
</style>
