# import pcbnew
import os
import shutil
import pathlib
import zipfile
import datetime

# 工程目录下Gerber和钻孔文件的存放位置
path_out = "kicad_gerber"
# 下单用的文件的位置
path_final = "jlc_gerber"

current_path = pathlib.Path(__file__).parent.absolute()
(head,tail) = os.path.split(current_path)
project_name = tail

# 用于检查文件是否为Gerber文件以判断是否进行替换操作
file_filter = ('.gbl','.gbs','.gbp','.gbo','.gm1','gm13',
               '.gtl','.gts','.gtp','.gto','.drl','.G1',
               '.G2','.gko','.g2','.g3','.g5','.g6')

jlc_header='''
G04 EasyEDA Pro v2.2.27.1, 2024-09-15 10:59:50*
G04 Gerber Generator version 0.3*
G04 Scale: 100 percent, Rotated: No, Reflected: No*
G04 Dimensions in millimeters*
G04 Leading zeros omitted, absolute positions, 3 integers and 5 decimals*
'''

jlc_header2='''
;EasyEDA Pro Client v2.2.27.1, 2024-09-15 10:59:51
;Gerber Generator version 0.3
'''

# 两张对应表，分别根据结尾和文件名来判断该给什么生成的文件什么名称
replace_list_end = [('.gbl',"Gerber_BottomLayer.GBL",           "BottomLayer"),
                    ('.gko',"Gerber_BoardOutlineLayer.GKO",     "BoardOutlineLayer"),
                    ('.gbp',"Gerber_BottomPasteMaskLayer.GBP",  "BottomPasteMaskLayer"),
                    ('.gbo',"Gerber_BottomSilkscreenLayer.GBO", "BottomSilkscreenLayer"),
                    ('.gbs',"Gerber_BottomSolderMaskLayer.GBS", "BottomSolderMaskLayer"),
                    ('.gtl',"Gerber_TopLayer.GTL",              "TopLayer"),
                    ('.gtp',"Gerber_TopPasteMaskLayer.GTP",     "TopPasteMaskLayer"),
                    ('.gto',"Gerber_TopSilkscreenLayer.GTO",    "TopSilkscreenLayer"),
                    ('.gts',"Gerber_TopSolderMaskLayer.GTS",    "TopSolderMaskLayer"),
                    ('.gd1',"Drill_Through.GD1",                "Drill_Through"),
                    ('.g2',"Gerber_InnerLayer1.GP1",            "InnerLayer1"),
                    ('.g3',"Gerber_InnerLayer2.GP2",            "InnerLayer2"),
#('.gm1',"Gerber_MechanicalLayer1.GM1"),
                    ('.gm13',"Gerber_MechanicalLayer13.GM13",   "MechanicalLayer13")]

replace_list_contain = [
    ('PTH',             'Drill_PTH_Through.DRL',            'PTH_Through',           'DRL'),
    ('NPTH',            'Drill_NPTH_Through.DRL',           'NPTH_Through',          'DRL'),
    ('front-in1',       'Drill_PTH_Top_to_Inner1.DRL',      'PTH_Top_to_Inner1',     'DRL'),
    ('front-in2',       'Drill_PTH_Top_to_Inner2.DRL',      'PTH_Top_to_Inner2',     'DRL'),
    ('In1_Cu',          'Gerber_InnerLayer1.GP1',           'InnerLayer1',           'GP1'),
    ('In2_Cu',          'Gerber_InnerLayer2.GP2',           'InnerLayer2',           'GP2'),
    ('Top Layer',       'Gerber_TopLayer.GTL',              'TopLayer',              'GTL'),
    ('Bottom Layer',    'Gerber_BottomLayer.GBL',           'BottomLayer',           'GBL'),
    ('Top Overlay',     'Gerber_TopSilkscreenLayer.GTO',    'TopSilkscreenLayer',    'GTO'),
    ('Bottom Overlay',  'Gerber_BottomSilkscreenLayer.GBO', 'BottomSilkscreenLayer', 'GBO'),
    ('Top Paste',       'Gerber_TopPasteMaskLayer.GTP',     'TopPasteMaskLayer',     'GTP'),
    ('Bottom Paste',    'Gerber_BottomPasteMaskLayer.GBP',  'BottomPasteMaskLayer',  'GBP'),
    ('Top Solder',      'Gerber_TopSolderMaskLayer.GTS',    'TopSolderMaskLayer',    'GTS'),
    ('Bottom Solder',   'Gerber_BottomSolderMaskLayer.GBS', 'BottomSolderMaskLayer', 'GBS'), 
    ('Edge_Cuts',       'Gerber_BoardOutlineLayer.GKO',     'BoardOutlineLayer',     'GKO')    
]

# save in txt
def save_in_text(log_path,strtext):
    with open(log_path,"a",encoding='utf-8') as file:
        file.write(f"{strtext}\n")
        file.close()
    return 0

def zipFolder(folder_path, output_path):
    """
    压缩指定路径下的文件夹
    :param folder_path: 要压缩的文件夹路径
    :param output_path: 压缩文件的输出路径
    """
    with zipfile.ZipFile(output_path, "w", zipfile.ZIP_DEFLATED) as zip:
        for root, dirs, files in os.walk(folder_path):
            for file in files:
                file_path = os.path.join(root, file)
                zip.write(file_path, os.path.relpath(file_path, folder_path))
    return 0

# 读取Gerber文件和钻孔文件，修改名称并给Gerber文件内容添加识别头后写入到输出文件夹
def fileTransform(filename, path_out):
    # 按行读取文件内容
    lines = open(filename).readlines()
    for replace_couple in replace_list_end:
        if filename.endswith(replace_couple[0]):
            file_new = open(path_out + '/' + replace_couple[1], 'w')
            file_new.write(f'G04 Layer: ' + replace_couple[2] + '*' + jlc_header)

            # 写入其他内容
            for line in lines:
                file_new.write(line)
            # 关闭文件
            file_new.close()

    for replace_couple in replace_list_contain:
        # 在替换列表内
        if filename.find(replace_couple[0]) != -1:
            # 新建文件
            file_new = open(path_out + '/' + replace_couple[1], 'w')
            # 写入开头
            if replace_couple[2] == 'PTH_Through' or replace_couple[2] == 'NPTH_Through' or replace_couple[2] == 'PTH_Top_to_Inner1' or replace_couple[2] == 'PTH_Top_to_Inner2':
                file_new.write(f';TYPE=PLATED\n;Layer: '+ replace_couple[2] + jlc_header2)
            else:
                file_new.write(f'G04 Layer: ' + replace_couple[2] + '*' + jlc_header)
                
            # 写入其他内容
            for line in lines:
                file_new.write(line)
            # 关闭文件
            file_new.close()
    return 0

def pathInit(path_out):
    # 检查下目录是否存在，没有就创建
    folder_out = os.path.exists(path_out)
    if not folder_out:
        os.makedirs(path_out)
        print("Folder %s created!" % path_out)
    else:
        print("Folder \"%s\" already exists!" % path_out)

    # 清空目录
    for files in os.listdir(path_out):
        path = os.path.join(path_out, files)
        try:
            shutil.rmtree(path)
        except OSError:
            os.remove(path)

    print("Folder \"%s\" clean!" % path_out)
    return 0


class GiveMeFreePCB():
    def Run(self):
        # 把工程根目录设为工作目录
        os.chdir(current_path)

        path_out_abs = os.path.join(os.getcwd(), path_out)
        # 创建 log.txt文件(记录)
        f = open(path_out_abs + '/' + 'log.txt' , 'w')
        f.close

        pathInit(os.path.join(current_path, path_final))

        file_count = 0

        path_files = os.listdir(os.path.join(os.getcwd(), path_out))

        # 遍历out目录下的文件，识别类型并进行相应的处理
        for p in path_files:
            if(os.path.isfile(os.path.join(path_out_abs, p))):
                if(p.endswith(file_filter)):
                    save_in_text(path_out_abs + '/' + 'log.txt', p)
                    fileTransform(os.path.join(path_out_abs, p), os.path.join(os.getcwd(), path_final))
                    file_count += 1

        timestamp = datetime.datetime.now().strftime('%Y%m%d%H%M%S')

        zipFolder(os.path.join(os.getcwd(), path_final) , os.getcwd() + '/' + "out_" + project_name + '-' + timestamp + ".zip")

        # 打开资源管理器
        # os.system("explorer.exe %s" % os.getcwd())

GiveMeFreePCB().Run()