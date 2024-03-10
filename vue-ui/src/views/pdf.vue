<template>
<div style="width:1100px;height:600px;border:1px green solid">
  <div class="about" style="float:left">
    <input type="file" @change="changeFile" />
    <div >
        <div class="box-scale" >
        <vue-office-pdf :src="pdfUrls" class="docx-calss" @rendered="rendered" />
        </div>

    </div>
  </div>
  <div style="float:left">
    <div>阅读笔记</div>
    <div>
        1. 预测 <button>解释</button><br/>
        2. 提问 <button>解释</button><br/>
        3. 澄清 <button>解释</button><br/>
        4. 想象 <button>解释</button><br/>
        5. 总结 <button>解释</button><br/>
        <el-button type="primary">Primary</el-button>
    </div>
  </div>
</div>
</template>

<script>
import VueOfficePdf from "@vue-office/pdf";


export default {
  name: 'pdfPreview2',
  data() {
    return {
      pdfUrls: "http://localhost:5173/Writing1.pdf"
    };
  },
  components: {
    VueOfficePdf,
  },
  methods: {
    rendered() {
      console.log('ddddddddd');
    },
    changeFile(event) { //也可以预览本地上传的pdf文件
      let _this = this;
      console.log(event.target.files[0]);
      const reader = new FileReader();
      reader.addEventListener('load', function() {
        _this.pdfUrls = reader.result;
      })
      reader.readAsDataURL(event.target.files[0]);
    }
  },
}
</script>>

<style scoped>
.about {
  width: 800px;
  height: 590px;
  border: 1px solid red;
  overflow: auto;
}
.box-scale {
  transform: scale(0.8);
  transform-origin: center top; /*设置缩放中心点*/
  transition: .2s;
}
.docx-calss {
  height: 100%;
}
</style>