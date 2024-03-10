<template>
  <div class="common-layout">
    <el-container>
      <el-header>
        <router-link to="/" style="margin-right: 10px;">Login</router-link>|
        <router-link to="/reg"  style="margin-right: 10px;">Register</router-link>|
        <router-link to="/pdf"  style="margin-right: 10px;">Pdf</router-link>|
        <router-link to="/pdf2"  style="margin-right: 10px;">Pdf2</router-link>
      </el-header>
      <el-main>
        <router-view></router-view>
      </el-main>
    </el-container>
  </div>
</template>