<script setup lang="ts">

</script>

<template>
    <div class="common-layout">
        <el-tabs type="border-card" class="demo-tabs">
            <el-tab-pane label="添加掉落物">
                <add-res></add-res>
            </el-tab-pane>
            <el-tab-pane label="添加活动">
                <add-activity></add-activity>
            </el-tab-pane>
            <el-tab-pane label="Role">Role</el-tab-pane>
            <el-tab-pane label="Task">Task</el-tab-pane>
        </el-tabs>
    </div>
</template>

<style scoped>
.demo-tabs>.el-tabs__content {
    padding: 32px;
    color: #6b778c;
    font-size: 32px;
    font-weight: 600;
}

.demo-tabs .custom-tabs-label .el-icon {
    vertical-align: middle;
}

.demo-tabs .custom-tabs-label span {
    vertical-align: middle;
    margin-left: 4px;
}
</style>
