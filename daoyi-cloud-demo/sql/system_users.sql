CREATE TABLE "demo".system_users
(
    id          bigserial PRIMARY KEY NOT NULL,
    username    varchar(30)           NOT NULL,
    password    varchar(100)          NOT NULL DEFAULT '',
    nickname    varchar(30)           NOT NULL,
    remark      varchar(500),
    dept_id     bigint,
    post_ids    varchar(255),
    email       varchar(50)                    DEFAULT '',
    mobile      varchar(11)                    DEFAULT '',
    sex         smallint                       DEFAULT 0,
    avatar      varchar(512)                   DEFAULT '',
    status      smallint              NOT NULL DEFAULT 0,
    login_ip    varchar(50)                    DEFAULT '',
    login_date  timestamp,
    creator     varchar(64)                    DEFAULT '',
    create_time timestamp             NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(64)                    DEFAULT '',
    update_time timestamp             NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean               NOT NULL DEFAULT false,
    tenant_id   bigint                NOT NULL DEFAULT 0
);

-- 添加表注释
COMMENT
ON TABLE "demo".system_users IS '用户信息表';

-- 添加列注释
COMMENT
ON COLUMN "demo".system_users.id IS '用户ID';
COMMENT
ON COLUMN "demo".system_users.username IS '用户账号';
COMMENT
ON COLUMN "demo".system_users.password IS '密码';
COMMENT
ON COLUMN "demo".system_users.nickname IS '用户昵称';
COMMENT
ON COLUMN "demo".system_users.remark IS '备注';
COMMENT
ON COLUMN "demo".system_users.dept_id IS '部门ID';
COMMENT
ON COLUMN "demo".system_users.post_ids IS '岗位编号数组';
COMMENT
ON COLUMN "demo".system_users.email IS '用户邮箱';
COMMENT
ON COLUMN "demo".system_users.mobile IS '手机号码';
COMMENT
ON COLUMN "demo".system_users.sex IS '用户性别';
COMMENT
ON COLUMN "demo".system_users.avatar IS '头像地址';
COMMENT
ON COLUMN "demo".system_users.status IS '帐号状态（0正常 1停用）';
COMMENT
ON COLUMN "demo".system_users.login_ip IS '最后登录IP';
COMMENT
ON COLUMN "demo".system_users.login_date IS '最后登录时间';
COMMENT
ON COLUMN "demo".system_users.creator IS '创建者';
COMMENT
ON COLUMN "demo".system_users.create_time IS '创建时间';
COMMENT
ON COLUMN "demo".system_users.updater IS '更新者';
COMMENT
ON COLUMN "demo".system_users.update_time IS '更新时间';
COMMENT
ON COLUMN "demo".system_users.deleted IS '是否删除';
COMMENT
ON COLUMN "demo".system_users.tenant_id IS '租户编号';

INSERT INTO "demo".system_users (username, password, nickname, remark, dept_id,
                                 post_ids, email, mobile, sex, avatar, status,
                                 login_ip, login_date, creator, create_time,
                                 updater, update_time, deleted, tenant_id)
VALUES ('achuan',
        '$argon2id$v=19$m=19456,t=2,p=1$oypeWR6gXDW5Im4eetcj3A$8dZ/wReNgOlxHgAIQhMCMKUWlPvk+kLcLCjc7sL+gDY',
        '阿川',
        '',
        100,
        '[4]',
        'gemiman@sina.com',
        '13787780026',
        1,
        '',
        0,
        '',
        NULL,
        '1',
        '2024-11-19 16:13:56',
        '1',
        '2025-06-02 00:42:40',
        false,
        1);