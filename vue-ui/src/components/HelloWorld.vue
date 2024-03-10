<script setup lang="ts">
import { ref } from 'vue'
import axios from 'axios'


defineProps<{ msg: string }>()

const count = ref(0)

function getData(){
  alert("get data from end")
  axios.get('http://localhost:3000/').then(function (response) {
    alert(response.data);
  }).catch(function (error) {
    alert(error);
  });

}

</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="count++">count is {{ count }}</button>
    
    <button type="button" @click="getData()">get data from end</button>

    <div class="mb-4">
      <el-button>Default</el-button>
      <el-button type="primary">Primary</el-button>
      <el-button type="success">Success</el-button>
      <el-button type="info">Info</el-button>
      <el-button type="warning">Warning</el-button>
      <el-button type="danger">Danger</el-button>
    </div>

    <p>
      “四个关系”：人与自我、人与他人、人与社会、人与自然之间的关系。<br/>
      “五个层面”：时代、社会、生活、人文、哲理五个层面。<br/>
      哲理思辨题、人与自然题、价值观念题、诗词格言题、文化现象题、生活情景题。
    </p>
  </div>

</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
