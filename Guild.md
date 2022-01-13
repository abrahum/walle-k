# 关于频道（Guild）相关事件以及动作

目前关于频道在 V12 的相关支持还是空白，根据之前的大概意向，是在 Group 以外增加一套适用于 Guild 和 Channel 的事件和动作。

以下对比一下开黑啦与 QQ (引用数据来自二者公开文件):

> 最后一栏用于表示 Onebot 是否应该或如何处理该字段（仅作提议，欢迎讨论）
> 
> - Y: Yes
> 
> - N: No
> 
> - D: Detail
> 
> - O: Option

### Objects

- Guild
  
  | Field        | Type   | QQ  | KHL       | OneBot | 备注        |
  |:------------:|:------:|:---:|:---------:|:------:|:---------:|
  | id           | string | Y   | Y         | Y      | 频道ID      |
  | name         | string | Y   | Y         | Y      | 频道名称      |
  | icon         | string | Y   | Y         | Y      | 频道头像地址    |
  | owner_id     | string | Y   | master_id | D      | 创建人用户ID   |
  | owner        | bool   | Y   | N         | O      | 当前人是否是创建人 |
  | member_count | int    | Y   | N         | O      | 成员数       |
  | max_members  | int    | Y   | N         | O      | 最大成员数     |
  | description  | string | Y   | topic?    | D      | 描述        |
  | joined_at    | string | Y   | N         | O      | 加入时间      |
  | ...          |        |     |           |        |           |

- Channel
  
  | Field        | Type   | QQ  | KHL          | OneBot | 备注      |
  |:------------:|:------:|:---:|:------------:|:------:|:-------:|
  | id           | string | Y   | Y            | Y      | 子频道 id  |
  | guild_id     | string | Y   | Y            | Y      | 频道 id   |
  | name         | string | Y   | Y            | Y      | 子频道名    |
  | type         | int    | Y   | Y            | D      | 子频道类型   |
  | sub_type     | int    | Y   | N            | O      | 子频道子类型  |
  | position     | int    | Y   | level        | O      | 排序      |
  | parent_id    | string | Y   | Y            | D      | 所属分组 id |
  | owner_id     | string | Y   | user_id(...) | D      | 创建人 id  |
  | private_type | int    | Y   | N            | O      | 子频道私密类型 |
  | ...          |        |     |              |        |         |

- User: 与现有 User 保持一致即可

### 事件

| 事件      | <center>Onebot</center> | QQ             | KHL | 备注                    |
|:-------:|:----------------------- |:-------------- |:---:|:---------------------:|
| 子频道消息事件 | MessageEvent            | 官方似乎只推送 At 消息？ | Y   | detail_type: channel? |
| ...     |                         |                |     |                       |

### 动作

| 动作                   | QQ  | KHL | 备注  |
|:-------------------- |:---:|:---:| --- |
| get_guild_info       | Y   | Y   |     |
| get_guild_list       | Y   | Y   |     |
| get_channel_info     | Y   | Y   |     |
| get_channel_list     | Y   | Y   |     |
| send_channel_message | Y   | Y   |     |
| set_channel_mute     | Y   | Y   |     |
| ...                  |     |     |     |



> 还有身份组（Role）需要讨论是否在标准内支持

抛砖引玉，欢迎大家讨论